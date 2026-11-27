sandbox=$(mktemp -d)
trap 'rm -rf "$sandbox"' EXIT
export REGISTRY="$sandbox/issue-number"
printf '0\n' > "$REGISTRY"
mkdir -p "$sandbox/bin" "$sandbox/first/questions" "$sandbox/second/questions"
export PATH="$sandbox/bin:$PATH"
cat > "$sandbox/bin/gh" <<'SCRIPT'
#!/bin/sh
set -eu
[ "$1" = api ] && [ "$2" = repos/earth-metabolome-initiative/competency-questions/issues ] || exit 1
[ "${DENY_ISSUES:-0}" = 0 ] || exit 1
number=$(( $(cat "$REGISTRY") + 1 ))
printf '%s\n' "$number" | tee "$REGISTRY"
SCRIPT
chmod +x "$sandbox/bin/gh"
for checkout in first second; do
    printf '# Formulas\n\n## CQ-0014\n\nOriginal question?\n' > "$sandbox/$checkout/questions/molecular-formulas.md"
done
cd "$sandbox/first"
file=questions/molecular-formulas.md
cp "$file" "$sandbox/original"
if "$CATALOGUE" add ../outside 'Question?'; then exit 1; fi
if "$CATALOGUE" add molecular-formulas '  '; then exit 1; fi
if DENY_ISSUES=1 "$CATALOGUE" add molecular-formulas 'Unavailable?'; then exit 1; fi
cmp "$file" "$sandbox/original"
[ "$(cat "$REGISTRY")" = 0 ]
"$CATALOGUE" add molecular-formulas 'Exactly two chlorine atoms?' > "$sandbox/result"
grep -F 'CQ-0015' "$sandbox/result"
grep -F "$file" "$sandbox/result"
grep -Fx 'Original question?' "$file"
grep -Fx '## CQ-0015' "$file"
grep -Fx 'Exactly two chlorine atoms?' "$file"
grep -F 'https://github.com/earth-metabolome-initiative/competency-questions/issues/1' "$file"
cd "$sandbox/second"
"$CATALOGUE" add molecular-formulas 'Exactly two bromine atoms?'
grep -Fx '## CQ-0016' "$file"
if grep -Fq 'CQ-0015' "$file"; then exit 1; fi
