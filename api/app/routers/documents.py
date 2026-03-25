"""Document generation, validation, rendering, parsing, and extraction endpoints."""

from __future__ import annotations

from fastapi import APIRouter, UploadFile
from fastapi.responses import Response

from app.errors import SpdfApiError, handle_engine_error
from app.schemas import (
    GenerateRequest,
    HealthResponse,
    InvoiceData,
    ParseRequest,
    ParseResponse,
    ValidationReport,
)
from app.services.spdf_engine import SpdfEngine

router = APIRouter(prefix="/api/v1", tags=["documents"])

MAX_UPLOAD_BYTES = 100 * 1024 * 1024  # 100 MB
ZIP_MAGIC = b"PK\x03\x04"


async def read_upload(file: UploadFile) -> bytes:
    """Read an uploaded file with size enforcement and ZIP magic validation."""
    chunks: list[bytes] = []
    total = 0

    while True:
        chunk = await file.read(1024 * 1024)  # 1 MB chunks
        if not chunk:
            break
        total += len(chunk)
        if total > MAX_UPLOAD_BYTES:
            raise SpdfApiError(
                413, "FILE_TOO_LARGE", f"Upload exceeds {MAX_UPLOAD_BYTES} byte limit."
            )
        chunks.append(chunk)

    data = b"".join(chunks)
    if len(data) < 4 or data[:4] != ZIP_MAGIC:
        raise SpdfApiError(
            400, "INVALID_CONTAINER", "File is not a valid SPDF container (bad magic bytes)."
        )
    return data


# --- Endpoints ---


@router.get("/health", response_model=HealthResponse)
async def health() -> HealthResponse:
    """Health check with engine version."""
    try:
        engine_version = SpdfEngine.version()
    except Exception:
        engine_version = "unavailable"
    return HealthResponse(engine_version=engine_version)


@router.post("/documents/generate")
async def generate_document(req: GenerateRequest) -> Response:
    """Build an SPDF container from layer JSON dicts."""
    try:
        spdf_bytes = SpdfEngine.generate(
            semantic=req.semantic,
            layout=req.layout,
            styles=req.styles,
            metadata=req.metadata,
            audit=req.audit,
        )
    except ValueError as exc:
        raise handle_engine_error(exc) from exc

    return Response(
        content=spdf_bytes,
        media_type="application/octet-stream",
        headers={"Content-Disposition": 'attachment; filename="document.spdf"'},
    )


@router.post("/documents/validate", response_model=ValidationReport)
async def validate_document(file: UploadFile) -> ValidationReport:
    """Validate an uploaded SPDF container."""
    data = await read_upload(file)
    try:
        report = SpdfEngine.validate(data)
    except ValueError as exc:
        raise handle_engine_error(exc) from exc
    return ValidationReport(**report)


@router.post("/documents/render")
async def render_document(file: UploadFile) -> Response:
    """Render the semantic layer of an SPDF container to PDF."""
    data = await read_upload(file)
    try:
        pdf_bytes = SpdfEngine.render_pdf(data)
    except ValueError as exc:
        raise handle_engine_error(exc) from exc

    return Response(
        content=pdf_bytes,
        media_type="application/pdf",
        headers={"Content-Disposition": 'inline; filename="document.pdf"'},
    )


@router.post("/documents/parse", response_model=ParseResponse)
async def parse_semantic(req: ParseRequest) -> ParseResponse:
    """Parse and validate a semantic JSON string."""
    try:
        doc = SpdfEngine.parse(req.semantic_json)
    except ValueError as exc:
        raise handle_engine_error(exc) from exc
    return ParseResponse(document=doc)


@router.post("/documents/extract", response_model=InvoiceData)
async def extract_invoice(file: UploadFile) -> InvoiceData:
    """Extract structured invoice data from an SPDF container."""
    data = await read_upload(file)
    try:
        invoice = SpdfEngine.extract(data)
    except ValueError as exc:
        raise handle_engine_error(exc) from exc
    return InvoiceData(**invoice)
