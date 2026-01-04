clean:
	docker stop pt-db
	rm -rf ./database/db/*
	docker rm pt-db
