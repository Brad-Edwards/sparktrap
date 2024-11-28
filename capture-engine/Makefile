build:
    docker-compose build

start:
    docker-compose up -d

stop:
    docker-compose down

logs:
    docker-compose logs -f

rebuild:
    docker-compose build capture
    docker-compose up -d capture

clean:
    docker-compose down -v
    docker system prune -f