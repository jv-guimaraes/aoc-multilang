const std = @import("std");
const input = @embedFile("input.txt");

pub fn main() !void {
    var lines = std.mem.split(u8, input, "\r\n");

    var all_sums = std.ArrayList(i32).init(std.heap.page_allocator);
    defer all_sums.deinit();

    var current_sum: i32 = 0;
    var max_sum: i32 = 0;
    while (lines.next()) |line| {
        const weight = std.fmt.parseInt(i32, line, 10) catch -1;
        if (weight == -1) {
            if (current_sum > max_sum) {
                max_sum = current_sum;
            }
            try all_sums.append(current_sum);
            current_sum = 0;
        } else {
            current_sum += weight;
        }
    }
    std.debug.print("Part 1: {}\n", .{max_sum});

    std.sort.sort(i32, all_sums.items, {}, std.sort.asc(i32));
    std.debug.print("Part 2: {}\n", .{all_sums.pop() + all_sums.pop() + all_sums.pop()});
}
