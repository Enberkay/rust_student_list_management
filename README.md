# Student Management System

A full-stack student management application with Rust (Actix-web) backend and Next.js frontend.

## Project Structure

```
rust_student_list_management/
├── backend/                 # Rust + Actix-web API
│   ├── src/
│   │   └── main.rs         # API server
│   ├── Cargo.toml          # Rust dependencies
│   └── Cargo.lock
├── frontend/               # Next.js frontend
│   ├── src/
│   │   ├── app/           # Next.js app router
│   │   ├── components/    # React components
│   │   └── lib/          # API service
│   ├── package.json
│   └── next.config.js
└── README.md
```

## Features

- **Backend (Rust + Actix-web)**:
  - RESTful API endpoints
  - Student CRUD operations
  - Statistics calculation
  - JSON serialization/deserialization

- **Frontend (Next.js)**:
  - Modern React with TypeScript
  - Tailwind CSS for styling
  - Responsive design
  - Real-time data updates

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Node.js (18+)
- npm or yarn

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Install dependencies and run the server:
   ```bash
   cargo run
   ```

The API server will start at `http://localhost:3001`

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run the development server:
   ```bash
   npm run dev
   ```

The frontend will start at `http://localhost:3000`

## API Endpoints

- `GET /api/students` - Get all students
- `POST /api/students` - Create a new student
- `GET /api/students/{id}` - Get a specific student
- `PUT /api/students/{id}` - Update a student
- `DELETE /api/students/{id}` - Delete a student
- `GET /api/statistics` - Get statistics

## Development

### Backend Development

The backend uses Actix-web framework with the following features:
- Async/await support
- JSON serialization with serde
- UUID generation for student IDs
- Thread-safe data management with Mutex

### Frontend Development

The frontend uses Next.js 14 with:
- App Router
- TypeScript
- Tailwind CSS
- Client-side state management
- API integration with fetch

## Technologies Used

- **Backend**: Rust, Actix-web, Serde, UUID
- **Frontend**: Next.js, React, TypeScript, Tailwind CSS
- **Data Format**: JSON
- **Architecture**: RESTful API with SPA frontend
