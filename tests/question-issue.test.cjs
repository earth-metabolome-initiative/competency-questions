const test = require('node:test');
const assert = require('node:assert/strict');
const submit = require('../.github/scripts/question-issue.cjs');

const repo = { owner: 'example', repo: 'catalogue' };
const context = {
  repo, sha: 'snapshot',
  payload: {
    repository: { default_branch: 'catalogue-draft' },
    issue: {
      number: 2, title: 'Which formulas contain sodium?',
      body: '### Pick a topic\r\n\r\nCounting atoms in a formula\r\n\r\n### Give one example\r\n\r\nNaCl should match. HCl should not.',
    },
  },
};

test('an unrelated issue, a legacy command issue or a malformed form makes no writes', async () => {
  for (const body of ['A typo in the README', 'Question set: `elements`\n\nWhich atoms?',
    context.payload.issue.body.replace('Counting atoms in a formula', '../README'),
    context.payload.issue.body.replace('NaCl should match. HCl should not.', '_No response_')]) {
    await submit({}, { ...context, payload: { ...context.payload, issue: { number: 2, title: 'Question?', body } } });
  }
});

test('a submission and a retry after a PR failure produce one entry and one linked proposal', async () => {
  const branch = 'question/CQ-0016';
  const contents = new Map([['snapshot', '# Formulas\n\n## CQ-0007\n\nFluorine?\n']]);
  let proposals = [], attempts = 0, edits = 0;
  const client = {
    pulls: {
      list: async ({ head }) => { assert.equal(head, `example:${branch}`); return { data: proposals }; },
      create: async proposal => {
        assert.equal(proposal.base, 'catalogue-draft');
        assert.equal(proposal.head, branch);
        assert.match(proposal.body, /Closes #2\b/);
        if (++attempts === 1) throw new Error('temporary outage');
        proposals.push(proposal);
      },
    },
    git: {
      getRef: async () => { if (!contents.has(branch)) throw Object.assign(new Error('missing'), { status: 404 }); },
      createRef: async ({ ref, sha }) => { assert.equal(ref, `refs/heads/${branch}`); contents.set(branch, contents.get(sha)); },
    },
    repos: {
      getContent: async ({ path, ref }) => {
        assert.equal(path, 'questions/molecular-formulas.md');
        return { data: { content: Buffer.from(contents.get(ref)).toString('base64'), sha: 'file-version' } };
      },
      createOrUpdateFileContents: async request => {
        assert.equal(request.branch, branch);
        assert.equal(request.sha, 'file-version');
        edits++;
        contents.set(branch, Buffer.from(request.content, 'base64').toString());
      },
    },
  };
  await assert.rejects(submit(client, context), /temporary outage/);
  await submit(client, context);
  await submit(client, context);
  assert.equal(edits, 1);
  assert.equal(proposals.length, 1);
  assert.match(contents.get(branch), /Fluorine\?\n\n## CQ-0016\n\nWhich formulas contain sodium\?/);
  assert.match(contents.get(branch), /NaCl should match\. HCl should not\./);
  assert.match(contents.get(branch), /https:\/\/github\.com\/example\/catalogue\/issues\/2/);
  assert.doesNotMatch(contents.get('snapshot'), /sodium/);
});
