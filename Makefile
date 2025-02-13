setup_test_db:
    DATABASE_URL=postgres://testuser:testuser_p@localhost:8999/test_alpha diesel database setup

reset_test_db:
    DATABASE_URL=postgres://testuser:testuser_p@localhost:8999/test_alpha diesel database reset

migrate_test_db:
    DATABASE_URL=postgres://testuser:testuser_p@localhost:8999/test_alpha diesel migration run
