"""SPDF API — FastAPI application entry point.

Run with: uvicorn app.main:app --reload --port 8000
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.errors import register_exception_handlers
from app.middleware.rate_limit import RateLimitMiddleware
from app.routers import account, billing, documents, templates, webhooks
from app.services.api_keys import seed_test_user

app = FastAPI(
    title="SPDF API",
    version="0.1.0",
    description="Structured PDF document generation, validation, and extraction API",
)

app.add_middleware(RateLimitMiddleware)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

register_exception_handlers(app)
app.include_router(documents.router)
app.include_router(account.router)
app.include_router(billing.router)
app.include_router(webhooks.router)
app.include_router(templates.router)

seed_test_user()
