"""Tests for template CRUD endpoints."""

from __future__ import annotations

from fastapi.testclient import TestClient

from app.services.templates import template_store


# ---------------------------------------------------------------------------
# POST /api/v1/templates (create)
# ---------------------------------------------------------------------------


def test_create_template(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/templates",
        json={"name": "Invoice GST", "description": "GST invoice template"},
    )
    assert resp.status_code == 201
    body = resp.json()
    assert body["name"] == "Invoice GST"
    assert body["description"] == "GST invoice template"
    assert "id" in body
    assert "created_at" in body
    assert "updated_at" in body


def test_create_template_with_semantic(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/templates",
        json={
            "name": "Simple",
            "semantic_template": {"pages": [], "locale": "en-IN"},
        },
    )
    assert resp.status_code == 201
    assert resp.json()["semantic_template"]["locale"] == "en-IN"


def test_create_template_missing_name_returns_422(client: TestClient) -> None:
    resp = client.post("/api/v1/templates", json={"description": "no name"})
    assert resp.status_code == 422


def test_create_template_empty_name_returns_422(client: TestClient) -> None:
    resp = client.post("/api/v1/templates", json={"name": "  "})
    assert resp.status_code == 422


def test_create_template_401_without_auth(raw_client: TestClient) -> None:
    resp = raw_client.post("/api/v1/templates", json={"name": "Test"})
    assert resp.status_code == 401


# ---------------------------------------------------------------------------
# GET /api/v1/templates/{id}
# ---------------------------------------------------------------------------


def test_get_template_by_id(client: TestClient) -> None:
    create_resp = client.post("/api/v1/templates", json={"name": "Fetch Me"})
    tid = create_resp.json()["id"]
    resp = client.get(f"/api/v1/templates/{tid}")
    assert resp.status_code == 200
    assert resp.json()["name"] == "Fetch Me"


def test_get_template_not_found(client: TestClient) -> None:
    resp = client.get("/api/v1/templates/nonexistent-id")
    assert resp.status_code == 404


# ---------------------------------------------------------------------------
# GET /api/v1/templates (list)
# ---------------------------------------------------------------------------


def test_list_templates_empty(client: TestClient) -> None:
    resp = client.get("/api/v1/templates")
    assert resp.status_code == 200
    body = resp.json()
    assert body["items"] == []
    assert body["has_more"] is False


def test_list_templates_returns_items(client: TestClient) -> None:
    client.post("/api/v1/templates", json={"name": "A"})
    client.post("/api/v1/templates", json={"name": "B"})
    resp = client.get("/api/v1/templates")
    assert resp.status_code == 200
    assert len(resp.json()["items"]) == 2


def test_list_templates_pagination(client: TestClient) -> None:
    for i in range(5):
        client.post("/api/v1/templates", json={"name": f"T{i}"})
    resp = client.get("/api/v1/templates?limit=2")
    body = resp.json()
    assert len(body["items"]) == 2
    assert body["has_more"] is True
    assert body["next_cursor"] is not None
    resp2 = client.get(f"/api/v1/templates?limit=2&cursor={body['next_cursor']}")
    body2 = resp2.json()
    assert len(body2["items"]) == 2
    assert body2["items"][0]["name"] != body["items"][0]["name"]


# ---------------------------------------------------------------------------
# PATCH /api/v1/templates/{id}
# ---------------------------------------------------------------------------


def test_update_template_name(client: TestClient) -> None:
    create_resp = client.post("/api/v1/templates", json={"name": "Old Name"})
    tid = create_resp.json()["id"]
    resp = client.patch(f"/api/v1/templates/{tid}", json={"name": "New Name"})
    assert resp.status_code == 200
    assert resp.json()["name"] == "New Name"


def test_update_template_not_found(client: TestClient) -> None:
    resp = client.patch("/api/v1/templates/bad-id", json={"name": "X"})
    assert resp.status_code == 404


# ---------------------------------------------------------------------------
# DELETE /api/v1/templates/{id}
# ---------------------------------------------------------------------------


def test_delete_template(client: TestClient) -> None:
    create_resp = client.post("/api/v1/templates", json={"name": "Delete Me"})
    tid = create_resp.json()["id"]
    resp = client.delete(f"/api/v1/templates/{tid}")
    assert resp.status_code == 204
    get_resp = client.get(f"/api/v1/templates/{tid}")
    assert get_resp.status_code == 404


def test_delete_template_not_found(client: TestClient) -> None:
    resp = client.delete("/api/v1/templates/bad-id")
    assert resp.status_code == 404


def test_deleted_template_excluded_from_list(client: TestClient) -> None:
    create_resp = client.post("/api/v1/templates", json={"name": "Ghost"})
    tid = create_resp.json()["id"]
    client.delete(f"/api/v1/templates/{tid}")
    resp = client.get("/api/v1/templates")
    assert all(t["id"] != tid for t in resp.json()["items"])
