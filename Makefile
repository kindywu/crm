export:
	@pg_dump "postgres://postgres:dk82Fi2A_2d@localhost:5432/stats" --table=export_user_stats --data-only --column-inserts > data.sql
