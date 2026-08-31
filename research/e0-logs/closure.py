import hashlib, os, re, sys, json

BASE = os.path.abspath(".")
OC   = os.path.join(BASE, "OmegaClaw-Core")
PET  = os.path.join(BASE, "PeTTa")
CHR  = os.path.join(BASE, "petta_lib_chromadb")

IMPORT_RE = re.compile(r'\(\s*import!\s+&self\s+\(\s*library\s+([^)]*)\)\s*\)')
GITIMP_RE = re.compile(r'\(\s*git-import!\s+"([^"]+)"\s*\)')
EXTS = ["", ".metta", ".py", ".pl"]

def sha(p):
    with open(p, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()

def resolve(tokens):
    """Mirror PeTTa's (library ...) resolution against the on-disk layout."""
    if len(tokens) == 1:
        roots, rel = [os.path.join(PET, "lib")], tokens[0]
    else:
        repo, rel = tokens[0], tokens[1]
        root = {"OmegaClaw-Core": OC, "petta_lib_chromadb": CHR, "PeTTa": PET}.get(repo)
        if root is None:
            return None
        roots = [root, os.path.join(root, "lib")]
    rel = rel.lstrip("./")
    for root in roots:
        for e in EXTS:
            c = os.path.join(root, rel + e)
            if os.path.isfile(c):
                return c
    return None

seen, unresolved, gitimports = {}, [], []
queue = [os.path.join(OC, "run.metta"), os.path.join(OC, "lib_omegaclaw.metta")]

while queue:
    path = queue.pop(0)
    if path in seen or not os.path.isfile(path):
        continue
    seen[path] = sha(path)
    if not path.endswith(".metta"):
        continue
    text = open(path, encoding="utf-8", errors="replace").read()
    for url in GITIMP_RE.findall(text):
        gitimports.append((os.path.relpath(path, BASE), url))
    for raw in IMPORT_RE.findall(text):
        tokens = raw.split()
        target = resolve(tokens)
        if target:
            queue.append(target)
        else:
            unresolved.append((os.path.relpath(path, BASE), raw.strip()))

print(f"# Import closure — {len(seen)} files resolved\n")
for p in sorted(seen):
    print(f"{seen[p]}  {os.path.relpath(p, BASE)}")
print(f"\n# git-import! calls ({len(gitimports)}) — no revision in the call itself")
for src, url in gitimports:
    print(f"  {src}: {url}")
print(f"\n# Unresolved imports ({len(unresolved)})")
for src, raw in unresolved:
    print(f"  {src}: (library {raw.strip()})")
by_ext = {}
for p in seen:
    by_ext[os.path.splitext(p)[1] or "<none>"] = by_ext.get(os.path.splitext(p)[1] or "<none>", 0) + 1
print(f"\n# By extension: {by_ext}")
