## Database Separation

### Auth & Access Database (auth_db)

Responsible for authentication, authorization, user/team/project management, and billing.

**Tables:**

#### `users`

- `id` UUID PRIMARY KEY
- `email` TEXT UNIQUE NOT NULL
- `password_hash` TEXT NOT NULL
- `name` TEXT
- `plan_id` UUID NULL
- `created_at` TIMESTAMP DEFAULT now()
- `is_deleted` BOOLEAN DEFAULT FALSE

#### `teams`

- `id` UUID PRIMARY KEY
- `name` TEXT NOT NULL
- `owner_id` UUID REFERENCES users(id)
- `created_at` TIMESTAMP DEFAULT now()

#### `team_members`

- `team_id` UUID REFERENCES teams(id) ON DELETE CASCADE
- `user_id` UUID REFERENCES users(id) ON DELETE CASCADE
- `role` TEXT CHECK (role IN ('admin', 'editor', 'viewer'))
- PRIMARY KEY (`team_id`, `user_id`)

#### `projects`

- `id` UUID PRIMARY KEY
- `title` TEXT NOT NULL
- `team_id` UUID REFERENCES teams(id) NULL ON DELETE SET NULL
- `created_by` UUID REFERENCES users(id)
- `created_at` TIMESTAMP DEFAULT now()
- `external_data_id` UUID NOT NULL -- links to data_db.project_data(id)

#### `project_memberships`

- `user_id` UUID REFERENCES users(id)
- `project_id` UUID REFERENCES projects(id)
- `role` TEXT CHECK (role IN ('owner', 'editor', 'viewer'))
- PRIMARY KEY (`user_id`, `project_id`)

#### `plans`

- `id` UUID PRIMARY KEY
- `name` TEXT NOT NULL
- `features` JSONB NOT NULL
- `price_cents` INTEGER

#### `billing_audit`

- `id` UUID PRIMARY KEY
- `user_id` UUID REFERENCES users(id)
- `event_type` TEXT CHECK (event_type IN ('charge', 'refund', 'upgrade', 'downgrade'))
- `amount_cents` INTEGER
- `timestamp` TIMESTAMP DEFAULT now()

---

### Measurement Data Database (data_db)

Responsible for project-specific measurement templates, structured records, and media.

**Tables:**

#### `project_data`

- `id` UUID PRIMARY KEY -- links to auth_db.projects.external_data_id
- `template_id` UUID REFERENCES templates(id)
- `created_at` TIMESTAMP DEFAULT now()

#### `templates`

- `id` UUID PRIMARY KEY
- `name` TEXT
- `schema_json` JSONB NOT NULL
- `created_by` UUID
- `created_at` TIMESTAMP DEFAULT now()

#### `records`

- `id` UUID PRIMARY KEY
- `project_id` UUID REFERENCES project_data(id) ON DELETE CASCADE
- `data_json` JSONB NOT NULL
- `version` INTEGER NOT NULL
- `updated_by` UUID
- `updated_at` TIMESTAMP DEFAULT now()

#### `media_files`

- `id` UUID PRIMARY KEY
- `record_id` UUID REFERENCES records(id) ON DELETE CASCADE
- `filename` TEXT NOT NULL
- `mime_type` TEXT
- `uploaded_by` UUID
- `created_at` TIMESTAMP DEFAULT now()

#### `record_audit_log` (optional)

- `id` UUID PRIMARY KEY
- `record_id` UUID REFERENCES records(id) ON DELETE CASCADE
- `user_id` UUID
- `action` TEXT CHECK (action IN ('create', 'update', 'delete'))
- `timestamp` TIMESTAMP DEFAULT now()
