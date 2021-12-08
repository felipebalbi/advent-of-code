#include <unistd.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define NUM 9
#define MAX_LINE 4096

static char input[MAX_LINE];
static uint64_t fishes[NUM];

static void next_day(void)
{
	uint64_t table[NUM];
	int i;

	memset(table, 0x00, sizeof(table));
	
	table[8] += fishes[0];
	table[6] += fishes[0];

	for (i = 1; i < NUM; i++)
		table[i-1] += fishes[i];

	memcpy(fishes, table, sizeof(fishes));
}

static void dump(void)
{
	int i;

	for (i = 0; i < NUM; i++)
		printf("%d: %d\n", i, fishes[i]);
	printf("----------------------------------\n");
}

static uint64_t sum(void)
{
	uint64_t total = 0;
	int i;

	for (i = 0; i < NUM; i++)
		total += fishes[i];

	return total;
}

int main(void)
{
	uint64_t num;
	ssize_t n;
	int i;

	n = read(STDIN_FILENO, input, MAX_LINE);

	for (i = 0; i < n; i += 2) {
		num = strtoull(input + i, NULL, 10);

		fishes[num]++;
	}

	for (i = 0; i < 80; i++)
		next_day();

	printf("total: %lld\n", sum());

	return 0;
}
