build:
	echo "running rust code building scripts..."
	python3 ./main.py

.PHONY: clean

clean:
	rm -rf ./generated/*