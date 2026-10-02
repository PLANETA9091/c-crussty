#!/usr/bin/env python3
import base64, json, subprocess
REPO = "PLANETA9091/c-crussty"
R1, R2 = 37027181039, 37027255131
DISP = (f"DISP | AG-498 w526 | w2048@r1136 2/2 queued @498[ab] runs {R1}+{R2} "
        f"s3000/dcp1500 | 2/2 204")
PATCH = ("PATCH_SUMMARY | AG-498 w526 | files=claims,work/AG-498 | "
         "idea=w2048@r1136 legal w-curve tail | ev=2/2 204 @f46b934f")
assert len(DISP) <= 120, (len(DISP), DISP)
assert len(PATCH) <= 120, (len(PATCH), PATCH)

def tok(): return open("/tmp/gh_token").read().strip()
def gh(method, path, body=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {tok()}",
           "-H", "Accept: application/vnd.github+json",
           f"https://api.github.com/repos/{REPO}/{path.lstrip('/')}"]
    if body is not None:
        with open("/tmp/gh_body_498f.json", "w") as f: json.dump(body, f)
        cmd += ["--data-binary", "@/tmp/gh_body_498f.json"]
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=90)
    try: return json.loads(p.stdout or "{}")
    except Exception: return {"_raw": (p.stdout or "")[:200]}

for a in range(8):
    d = gh("GET", "contents/SHARED_BOARD.md?ref=master")
    sha = d.get("sha")
    content = base64.b64decode(d.get("content", "")).decode()
    if "AG-498 w526 | 2/2" in content or "DISP | AG-498" in content:
        print("DISP already on board", flush=True); break
    new = content + ("" if content.endswith("\n") else "\n") + \
        "\n".join((DISP, PATCH)) + "\n"
    r = gh("PUT", "contents/SHARED_BOARD.md",
           {"message": "board: AG-498 disp/patch w2048@r1136 legal (wave-526)",
            "content": base64.b64encode(new.encode()).decode(),
            "sha": sha, "branch": "master"})
    if r.get("commit", {}).get("sha"):
        print(f"DISP/PATCH OK commit={r['commit']['sha'][:8]}", flush=True); break
    print(f"retry {a}: {str(r.get('message'))[:60]}", flush=True)

CLAIM_MD = open("/home/z/c-crussty/work/ag498_claim.md").read() \
    .replace("<a>", str(R1)).replace("<b>", str(R2))
for a in range(6):
    d = gh("GET", "contents/claims/AG-498.md?ref=master")
    if d.get("sha"):
        print("claims/AG-498.md exists", flush=True); break
    r = gh("PUT", "contents/claims/AG-498.md",
           {"message": "claims: AG-498 prereg w2048@r1136 legal (wave-526)",
            "content": base64.b64encode(CLAIM_MD.encode()).decode(),
            "branch": "master"})
    if r.get("commit", {}).get("sha"):
        print(f"CLAIMS PUT OK commit={r['commit']['sha'][:8]}", flush=True); break
    print(f"claims retry {a}: {str(r.get('message'))[:60]}", flush=True)
print("DONE", flush=True)
