# CuteSender

A lightweight full-stack microservice for sending warm, uplifting messages via email.

CuteSender combines a modern **Svelte 5** frontend with an asynchronous **Rust/Axum** backend and direct **SMTP** email delivery. The project is intentionally small and simple while still using production-oriented technologies and containerization.

## Features

* Fast async backend built with Rust, Axum and Tokio
* Modern frontend built with Svelte 5, TypeScript and Vite
* Email delivery through Gmail SMTP using Lettre
* HTML + plain text emails using `multipart/alternative`
* Dockerized with multi-stage builds and Docker Compose
* Nginx reverse proxy for frontend and API traffic
* Environment-based configuration for SMTP credentials

## Tech Stack

### Backend

* Rust
* Axum
* Tokio
* Lettre
* Serde
* Tower HTTP

### Frontend

* Svelte 5
* TypeScript
* Vite
* CSS

### Infrastructure

* Docker
* Docker Compose
* Nginx
* Pinggy for development tunneling

## Project Structure

```text
CuteSender/
├── cutesender-rust/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── Dockerfile
│
├── cutesender-web/
│   ├── src/
│   │   └── App.svelte
│   ├── nginx.conf
│   ├── package.json
│   └── Dockerfile
│
├── docker-compose.yml
├── .env.example
└── README.md
```

## Quick Start

### Prerequisites

Make sure you have:

* [Docker](https://www.docker.com/)
* [Docker Compose](https://docs.docker.com/compose/)
* A Gmail account with 2FA enabled
* A Gmail [App Password](https://myaccount.google.com/apppasswords)

### 1. Clone the repository

```bash
git clone <repository-url>
cd CuteSender
```

### 2. Configure environment variables

Copy the example environment file:

```bash
cp .env.example .env
```

Then edit `.env`:

```env
SMTP_USER=your_email@gmail.com
SMTP_PASS=your_16_char_app_password
PORT=3000
```

`SMTP_PASS` should be a Gmail App Password, not your regular Gmail password.

### 3. Start the application

Build and start the containers:

```bash
docker compose up --build -d
```

The application will be available at:

* Frontend: `http://localhost:8080`
* Backend API: `http://localhost:3000`
* Send endpoint: `POST /api/send`

## API

### Send Cute Mail

```http
POST /api/send
Content-Type: application/json
```

Request body:

```json
{
  "email": "recipient@example.com"
}
```

### Responses

| Status                      | Description                      |
| --------------------------- | -------------------------------- |
| `200 OK`                    | Email sent successfully          |
| `400 Bad Request`           | Invalid or missing email address |
| `500 Internal Server Error` | SMTP delivery failed             |

## How It Works

```text
┌──────────────┐
│   Svelte 5   │
│   Frontend   │
└──────┬───────┘
       │
       │ POST /api/send
       ▼
┌──────────────┐
│     Nginx    │
│ Reverse Proxy│
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Rust + Axum  │
│    Backend   │
└──────┬───────┘
       │
       │ SMTP
       ▼
┌──────────────┐
│  Gmail SMTP  │
└──────┬───────┘
       │
       ▼
     Email
```

The frontend collects the recipient's email address and sends it to the Rust API. The backend validates the request, creates the email using Lettre, and delivers it through Gmail SMTP.

## Docker

CuteSender uses separate containers for the frontend and backend.

The backend container builds the Rust application and runs the Axum server.

The frontend container builds the Svelte application and serves the generated static files through Nginx.

Docker Compose orchestrates both services and provides the required networking between them.

## Development Tunnel

For external access during development, CuteSender can be exposed through a tunneling service such as Pinggy.

This is useful when testing the application from another device or sharing a temporary development instance.

## Security Notes

* Never commit `.env` to the repository.
* Never expose your Gmail App Password in frontend code.
* SMTP credentials are used exclusively by the backend.
* Use HTTPS when exposing the application publicly.
* Pinggy tunnels should be treated as development infrastructure, not as a production deployment solution.

## License

This project is provided for educational and experimental purposes.
