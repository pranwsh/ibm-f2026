from fastapi import FastAPI

app = FastAPI(title="Signal Pipeline API")


@app.get("/healthz")
def healthz() -> dict[str, str]:
    return {"status": "ok"}
