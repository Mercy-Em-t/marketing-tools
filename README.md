# Marketing Tools Platform

A centralized, intelligent platform for managing social media content, advertisements, customer interactions, and analytics across multiple platforms.

## Documentation

- [Software Requirements Specification (SRS)](docs/SRS.md)
- [System Architecture Document (SAD)](docs/SAD.md)
- [Design Specification (Technical Design)](docs/Design-Specification.md)

## Overview

This platform integrates with major social media networks (Instagram, Facebook, TikTok, X, WhatsApp) to enable:

- Ad creation, management, and optimization
- AI-powered content generation and scheduling
- Centralized messaging and customer interactions
- Analytics and performance insights
- Multi-user role-based access

## Tech Stack

- **Frontend:** Next.js
- **Backend:** Rust
- **Database:** Supabase

## Getting Started

### Backend MVP

```bash
cd backend
cargo run
```

Backend API starts on `http://127.0.0.1:3000` and currently implements MVP endpoints based on the design specification.
