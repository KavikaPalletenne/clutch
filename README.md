![ExamClutch logo](frontend/public/gradient_logo.svg "ExamClutch")

Open-source studying for everyone.

<img src="./frontend/public/homepage.svg" alt="Dashboard Mockup" width="600"/>

## Features
- Upload your notes and append tags.
- Search anyone's notes using full-text search.
- Retrieve and search notes directly within Discord.
- Use materials from trusted sources within your school.

___
## Architecture
### Backend
- Actix Web Server (CRUD, AuthZ)
- Meilisearch Search Server
- MongoDB Database
- S3 Compatible Storage (Files)

### Frontend
- NextJS

### Authentication
- Discord OAuth2
