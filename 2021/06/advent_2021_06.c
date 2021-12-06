#include <stdio.h>
#include <bits/stdint-uintn.h>
#include <stdlib.h>
#include <stdbool.h>
#include <memory.h>

// Length of the fishes by day array. Set this to the maximum day of the cycle of a fish plus 1
#define FISHES_BY_DAYS_LEN 9

void read_input(uint64_t* const fishes_by_day) {
    // Open input file
    FILE* input = fopen("input.txt", "r");
    if (!input) {
        perror("fopen input");
        exit(EXIT_FAILURE);
    }

    while (true) {
        // Read one char from the file, i.e. the days of a fish. All days are only one digit
        int digit = fgetc(input);
        if (feof(input)) {
            printf("Illegal format of input. Expected digit, got EOF\n");
            exit(EXIT_FAILURE);
        }
        // Convert ascii char to int
        int fish = digit - '0';
        // Add this fish to the count array
        fishes_by_day[fish]++;

        int comma = fgetc(input);
        if (comma != ',') {
            // No comma following -> completely read file
            break;
        }
    }
}

void simulate_day(uint64_t* const fishes_by_day) {
    // Save fishes that get rested to 6
    uint64_t fishesWithZeroDays = fishes_by_day[0];

    // Move all fishe groups one to the front, i.e. decrement their days
    memmove(fishes_by_day, fishes_by_day + 1, sizeof(uint64_t) * (FISHES_BY_DAYS_LEN - 1));

    // Reset fishes with zero days to 6 days.
    // Use += here because some fishes might already have 6 days
    fishes_by_day[6] += fishesWithZeroDays;
    // Overwrite day 8 with new born fishes because array[8] still contains the old value, because it is not
    // overwritten by memmove
    fishes_by_day[8] = fishesWithZeroDays;
}

int main(int argc, char* argv[]) {
    // The general idea is instead of calculating each fish on it is own,
    // calculate each group of fishes that are in the same day of their cycle.
    // This is necessary for part 2, because otherwise you would need terabytes of RAM for all the fishes.

    if (argc < 2) {
        printf("Usage: %s SIMULATION_DAYS\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    // Convert simulation days string to int
    int simulationDays = atoi(argv[1]);

    // Stores the number of fishes that are at day index in their cycle
    uint64_t fishes_by_day[FISHES_BY_DAYS_LEN] = {0};
    read_input(fishes_by_day);

    // Perform simulation for simulationDays many days
    for (int i = 0; i < simulationDays; i++) {
        simulate_day(fishes_by_day);
    }

    // Count all fishes
    uint64_t fishSum = 0;
    for (size_t i = 0; i < FISHES_BY_DAYS_LEN; i++) {
        fishSum += fishes_by_day[i];
    }

    // Print result
    printf("%lu\n", fishSum);

    return 0;
}
