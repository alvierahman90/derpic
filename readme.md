# derpic :camera_flash:

## Getting Started

1. Create a `.env` file with the following variables defined (edited as required):

   ```
   POSTGRES_PASSWORD=very_secure
   POSTGRES_USERNAME=postgres
   POSTGRES_HOST=db
   POSTGRES_DB=fel_dms
   DERPIC_ADMIN_TOKEN=changeme
   DERPIC_PUBLIC_BASE_URL=https://i.dev.alv.cx
   ```

2. `docker compose up -d --build`
