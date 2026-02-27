# 🚀 Dynamic Portfolio Server (Actor Architecture)

**Rust | Axum | PostgreSQL | Tokio Actors | Message-Driven System**

---

## 📌 Overview

Dynamic Portfolio Server is a high-performance backend built in **Rust** using an **Actor-Based Architecture** powered by Tokio channels.

Instead of traditional layered dependency injection, this version uses:

* 🧵 Independent **Actors**
* 📩 Message passing via `tokio::mpsc`
* 🔒 Isolated state per domain
* ⚡ Concurrent, non-blocking execution
* 🧩 Clear domain boundaries

Each domain (Auth, Blog, Project, Stack, Image, Refresh Token) runs as its own actor, processing messages asynchronously.

This architecture improves:

* Scalability
* Fault isolation
* Concurrency control
* Clean separation of responsibilities

---

# 🏗 Architecture Overview

```text
                ┌─────────────────────────┐
                │        HTTP Layer       │
                │      (Axum Router)      │
                └────────────┬────────────┘
                             │
                             ▼
                ┌─────────────────────────┐
                │        Handlers         │
                │  (Send Actor Messages)  │
                └────────────┬────────────┘
                             │
                ┌────────────▼────────────┐
                │      mpsc Channels      │
                └────────────┬────────────┘
                             │
     ┌───────────────┬───────────────┬───────────────┬───────────────┐
     ▼               ▼               ▼               ▼               ▼
 AuthActor     StackActor       BlogActor     ProjectActor    ImageActor
     ▼               ▼               ▼               ▼               ▼
 PostgreSQL      PostgreSQL      PostgreSQL      PostgreSQL      Cloudinary
```

---

# 🎭 What is Actor Architecture?

An **Actor**:

* Owns its internal state
* Receives messages
* Processes them sequentially
* Replies through a response channel
* Runs independently inside a Tokio task

This model eliminates shared mutable state and reduces race conditions.

---

# 📂 Project Structure

```text
api/                → Axum routing
auth/               → Auth actor + handlers + messages
blog/               → Blog actor + handlers + messages
project/            → Project actor + handlers + messages
stack/              → Stack actor + handlers + messages
image/              → Image actor (Cloudinary integration)
refresh_token/      → Refresh token actor + repo
config/             → Environment config
errors/             → Centralized error handling
state/              → AppState (channel senders)
utils/              → Utilities
core/               → Shared domain logic
```

---

# 🛠 Tech Stack

| Technology    | Purpose               |
| ------------- | --------------------- |
| 🦀 Rust       | Backend language      |
| ⚡ Axum        | Web framework         |
| 🧵 Tokio      | Async runtime         |
| 🐘 PostgreSQL | Database              |
| 🔄 SQLx       | Async DB driver       |
| ☁️ Cloudinary | Image hosting         |
| 🍪 Cookies    | Refresh token storage |

---

# 🚀 Actor System Initialization

Each domain has:

```rust
let (auth_tx, auth_rx) = mpsc::channel::<AuthMessage>(32);
tokio::spawn(AuthActor::new(pool.clone()).run(auth_rx));
```

Actors run independently and process messages continuously.

All senders are stored inside `AppState`:

```rust
pub struct AppState {
    pub auth_tx: Sender<AuthMessage>,
    pub stack_tx: Sender<StackMessage>,
    pub blog_tx: Sender<BlogMessage>,
    pub project_tx: Sender<ProjectMessage>,
    pub image_tx: Sender<ImageMessage>,
    pub refresh_token_tx: Sender<RefreshTokenMessage>,
}
```

Handlers send messages to actors instead of calling services directly.

---

# 🌐 Base URL

```
http://localhost:{PORT}/api/v1
```

---

# 🔐 Authentication APIs

### `/api/v1/auth`

| Method | Endpoint      | Description       |
| ------ | ------------- | ----------------- |
| POST   | `/register`   | Register new user |
| POST   | `/login`      | Login             |
| GET    | `/users`      | Get all users     |
| GET    | `/users/{id}` | Get single user   |
| PATCH  | `/users/{id}` | Update user       |
| DELETE | `/users/{id}` | Delete user       |

Uses secure cookies for refresh tokens.

---

# 🧠 Stack APIs

### `/api/v1/stack`

| Method | Endpoint            | Description        |
| ------ | ------------------- | ------------------ |
| POST   | `/create`           | Create stack       |
| GET    | `/all`              | Get all stacks     |
| GET    | `/by/{stack_title}` | Get stack by title |
| GET    | `/detail/{id}`      | Get stack by ID    |
| PATCH  | `/detail/{id}`      | Update stack       |
| DELETE | `/detail/{id}`      | Delete stack       |

---

# 🖼 Image APIs

### `/api/v1/image`

| Method | Endpoint  | Description           |
| ------ | --------- | --------------------- |
| POST   | `/base64` | Upload base64 image   |
| POST   | `/file`   | Upload form-data file |

Handled by `ImageActor`, isolated from database actors.

---

# 📝 Blog APIs

### `/api/v1/blog`

| Method | Endpoint       | Description   |
| ------ | -------------- | ------------- |
| POST   | `/create`      | Create blog   |
| GET    | `/all`         | Get all blogs |
| GET    | `/detail/{id}` | Get blog      |
| PATCH  | `/detail/{id}` | Update blog   |
| DELETE | `/detail/{id}` | Delete blog   |

---

# 💼 Project APIs

### `/api/v1/project`

| Method | Endpoint       | Description      |
| ------ | -------------- | ---------------- |
| POST   | `/create`      | Create project   |
| GET    | `/all`         | Get all projects |
| GET    | `/detail/{id}` | Get project      |
| PATCH  | `/detail/{id}` | Update project   |
| DELETE | `/detail/{id}` | Delete project   |

---

# 🔄 Token APIs

### `/api/v1/token`

| Method | Endpoint   | Description          |
| ------ | ---------- | -------------------- |
| POST   | `/refresh` | Refresh access token |
| POST   | `/logout`  | Logout user          |

Refresh tokens are handled by a dedicated `RefreshTokenActor`.

---

# ⚙️ Environment Variables

Create a `.env` file:

```env
DATABASE_URL=postgres://user:password@localhost:5432/db_name
PORT=8000

JWT_SECRET=your_super_secret
JWT_EXPIRY_HOUR=24

CLOUD_NAME=your_cloud_name
CLOUD_API_KEY=your_cloud_key
CLOUD_API_SECRET=your_cloud_secret

DB_POOL_MAX_CONNECTIONS=12
```

---

# 🚀 Running the Project

### 1️⃣ Clone

```bash
git clone https://github.com/your-username/dynamic-portfolio-actor.git
cd dynamic-portfolio-actor
```

### 2️⃣ Setup Database

```bash
sqlx database create
sqlx migrate run
```

### 3️⃣ Run

```bash
cargo run
```

Server starts at:

```
🚀 http://0.0.0.0:{PORT}
```

---

# 🎯 Why Actor Architecture?

### ✅ 1. Concurrency Without Shared State

Each actor:

* Owns its state
* Processes messages sequentially
* Eliminates data races

---

### ✅ 2. Fault Isolation

If one actor crashes:

* Others remain alive
* System degrades gracefully

---

### ✅ 3. Horizontal Scalability Ready

Actors can evolve into:

* Distributed services
* Event-driven architecture
* Kafka-backed message queues
* Microservices

---

### ✅ 4. Clean Domain Boundaries

Each domain module contains:

* Actor
* Messages
* Handlers
* Business logic

No cross-domain tight coupling.

---

# 🧠 Architectural Comparison

| Hexagonal             | Actor                               |
| --------------------- | ----------------------------------- |
| Layered               | Message-driven                      |
| Trait-based injection | Channel-based communication         |
| Direct service calls  | Async message passing               |
| Easier for CRUD apps  | Better for high concurrency systems |

---

# 🔥 Production Improvements

* Structured logging (Tracing)
* Circuit breaker per actor
* Graceful shutdown handling
* Message retry strategies
* Dead letter queues
* Distributed actor model
* Docker & Kubernetes deployment
* Redis for caching
* Observability dashboards

---

# 👨‍💻 Author

**Joshua**
Rust Backend Engineer
Systems Design | Concurrent Architecture | Distributed Systems Enthusiast

---

# 📜 License

MIT License

---

