# Thrifting API (Actix Web + SQLite)

A Rust backend for the Thrifting app, covering auth, products, wishlist, settings, stores, support, and FAQ, aligned with the existing frontend features.

## Features
- Auth: register, login (JWT)
- Products: list/filter/sort, CRUD (auth required for create/update/delete)
- Wishlist: add/remove/list (per user)
- Notifications: get/set user notification preferences
- Settings: profile and locale get/set
- Stores: list store locations
- FAQ: list items
- Support: submit support messages (logs)

## Quick start

1. Prerequisites: Rust toolchain, SQLite3.
2. Copy .env:

```bash
cp .env.example .env
```

3. Adjust CORS_ORIGIN and JWT_SECRET in .env.

4. Run database migrations automatically on startup.

5. Run server:

```bash
cargo run
```

Server runs on PORT (default 8080).

## API Overview

- POST /auth/register { email, password }
- POST /auth/login { email, password } -> { token }
- GET /products?category=&gender=&size=&color=&status=&sort=price_asc|price_desc
- GET /products/{id}
- POST /products (auth)
- PUT /products/{id} (auth)
- DELETE /products/{id} (auth)
- GET /wishlist (auth)
- POST /wishlist/{product_id} (auth)
- DELETE /wishlist/{product_id} (auth)
- GET /notifications/settings (auth)
- POST /notifications/settings (auth)
- GET /settings/profile (auth)
- POST /settings/profile (auth)
- GET /settings/locale (auth)
- POST /settings/locale (auth)
- GET /stores
- GET /faq
- POST /support/message

Auth: Use `Authorization: Bearer <token>` header for protected routes.

## Notes
- This is a minimal but complete baseline. You can map the frontend localStorage usage to these endpoints when you’re ready.
- Consider adding pagination, rate limiting, and production email/queue integrations later.
