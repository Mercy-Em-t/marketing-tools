# System Architecture Document (SAD)

**Project:** Marketing Tools Platform  
**Version:** 1.0  
**Date:** 2026-03-29  
**Status:** Draft

---

## 1.1 Overview

The system is a **modular, scalable, multi-tenant platform** for social media management, ad automation, and customer engagement.

It follows a:

> **Client → API → Services → Data → External APIs** architecture.

---

## 1.2 Architectural Style

- **Microservices-inspired modular monolith (MVP)**
- Evolves into **microservices** as system scales

This approach aligns with the current budget and timeline constraints.

---

## 1.3 High-Level Architecture

```text
[ Client (Web App - Next.js) ]
                ↓
        [ API Gateway (Rust) ]
                ↓
    --------------------------
    | Core Service Modules   |
    --------------------------
    | Ads Engine            |
    | Content Engine        |
    | Messaging Service     |
    | Analytics Engine      |
    | Automation Engine     |
    --------------------------
                ↓
     [ Database (PostgreSQL) ]
                ↓
     [ Storage + CDN Layer   ]
                ↓
 [ External Platform APIs ]
 (Meta, TikTok, X, WhatsApp)
```

---

## 1.4 Core Components

### 1. Frontend (Client Layer)

- Built with Next.js
- Handles:
  - UI/UX
  - State management
  - API communication

### 2. API Layer (Backend Entry Point)

- Rust-based (Axum/Actix)
- Responsibilities:
  - Authentication
  - Routing
  - Rate limiting
  - Request validation

### 3. Core Services Layer

#### a) Ads Service

- Campaign creation
- Performance tracking
- Optimization logic

#### b) Content Service

- Content generation (AI integration)
- Scheduling
- Templates

#### c) Messaging Service

- Unified inbox
- Message routing
- Admin approval flow

#### d) Analytics Service

- Aggregates metrics
- Generates reports
- Triggers alerts

#### e) Automation Engine

- Rule-based execution
- Event-driven triggers

### 4. Data Layer

- PostgreSQL (via Supabase)
- Stores:
  - Users
  - Campaigns
  - Messages
  - Analytics
  - Media references

### 5. Storage & CDN

- Media storage (images/videos)
- CDN for fast delivery
- Optimization layer

### 6. External Integrations

- Social Media APIs:
  - Meta (Instagram/Facebook)
  - TikTok
  - X
  - WhatsApp Business

---

## 1.5 Data Flow Example (Posting Content)

```text
User → Frontend → API → Content Service → Scheduler → Platform API → Publish
```

## 1.6 Data Flow Example (Message Handling)

```text
Platform → Webhook → API → Messaging Service → Inbox → Admin → Response → Platform
```

---

## 1.7 Scalability Strategy

- Start as monolith
- Introduce:
  - Background workers
  - Queue system (Redis/Kafka later)
- Move heavy modules (analytics, AI) to microservices

---

## 1.8 Security Architecture

- JWT authentication
- OAuth for platform integrations
- Role-based access control
- Encrypted storage (sensitive data)

---

## 1.9 Deployment Architecture

- Frontend: Vercel
- Backend: Cloud VM / container (Docker)
- Database: Supabase
- CDN: Cloudflare / Supabase Storage
