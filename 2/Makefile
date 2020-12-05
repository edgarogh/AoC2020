main-one: main.c
	$(CC) -DPART_ONE $^ -o $@

main-two: main.c
	$(CC) -DPART_TWO $^ -o $@

run%: main%
	./$<
