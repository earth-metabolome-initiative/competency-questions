const topics = new Map([
  ['Counting atoms in a formula', 'molecular-formulas'],
  ['Comparing whole molecules', 'smiles-rs'],
  ['Finding part of a molecule', 'smarts'],
  ['Elements and their isotopes', 'elements'],
]);

function plainText(text) {
  return text.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;')
    .replace(/[\\`*_\[\]#!|~]/g, '\\$&');
}

module.exports = async function submit(client, context) {
  const { issue, repository } = context.payload;
  const fields = (issue.body ?? '').split(/\r?\n/).map(line => line.trim()).filter(Boolean);
  const [topicHeading, topic, exampleHeading, example] = fields;
  const setName = topics.get(topic);
  if (fields.length !== 4 || topicHeading !== '### Pick a topic' || !setName ||
      exampleHeading !== '### Give one example' || example === '_No response_') return;
  const question = issue.title.trim();
  if (!question || /\p{Cc}/u.test(question) || /\p{Cc}/u.test(example)) return;

  const identifier = `CQ-${String(issue.number + 14).padStart(4, '0')}`;
  const branch = `question/${identifier}`;
  const path = `questions/${setName}.md`;
  const { owner, repo } = context.repo;
  const base = repository.default_branch;
  const proposals = await client.pulls.list({ owner, repo, head: `${owner}:${branch}`, base, state: 'all', per_page: 1 });
  if (proposals.data.length) return;

  try {
    await client.git.getRef({ owner, repo, ref: `heads/${branch}` });
  } catch (error) {
    if (error.status !== 404) throw error;
    await client.git.createRef({ owner, repo, ref: `refs/heads/${branch}`, sha: context.sha });
  }

  const { data: file } = await client.repos.getContent({ owner, repo, path, ref: branch });
  const contents = Buffer.from(file.content, 'base64').toString('utf8');
  const heading = `## ${identifier}`;
  if (!contents.split(/\r?\n/).includes(heading)) {
    const link = `https://github.com/${owner}/${repo}/issues/${issue.number}`;
    const entry = `${heading}\n\n${plainText(question)}\n\n**Example.** ${plainText(example)}\n\n[Review question](${link})\n`;
    await client.repos.createOrUpdateFileContents({ owner, repo, path, branch, sha: file.sha,
      message: `Add ${identifier} to ${setName}`, content: Buffer.from(`${contents.trimEnd()}\n\n${entry}`).toString('base64') });
  }
  await client.pulls.create({ owner, repo, base, head: branch, title: `Add ${identifier} to ${setName}`,
    body: `Please review the question and its example.\n\nCloses #${issue.number}` });
};
