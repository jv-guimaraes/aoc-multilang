const std = @import("std");
const input = @embedFile("input.txt");
const IntegerBitSet = std.bit_set.IntegerBitSet;

fn priority(char: u8) u8 {
    return switch (char) {
        97...122 => char - 96,
        else => char - 38,
    };
}

fn shared_item(sack: []const u8) ?u8 {
    const len = sack.len;
    var first_half = IntegerBitSet(123).initEmpty();

    for (sack[0 .. len / 2]) |char| {
        first_half.set(char);
    }

    for (sack[len / 2 ..]) |char| {
        if (first_half.isSet(char)) {
            return char;
        }
    }

    return null;
}

fn shared_badge(sack1: []const u8, sack2: []const u8, sack3: []const u8) ?u8 {
    var item_set1 = IntegerBitSet(123).initEmpty();
    var item_set2 = IntegerBitSet(123).initEmpty();
    for (sack1) |char| {
        item_set1.set(char);
    }
    for (sack2) |char| {
        item_set2.set(char);
    }
    for (sack3) |char| {
        if (item_set1.isSet(char) and item_set2.isSet(char)) {
            return char;
        }
    }
    return null;
}

pub fn main() !void {
    // Part 1
    var lines = std.mem.tokenize(u8, input, "\r\n");
    var sum: u32 = 0;
    while (lines.next()) |line| {
        const item = shared_item(line).?;
        sum += priority(item);
    }
    std.debug.print("Part 1: {}\n", .{sum});

    // Part 2
    lines.reset();
    sum = 0;
    while (lines.next()) |line1| {
        const line2 = lines.next().?;
        const line3 = lines.next().?;
        const item = shared_badge(line1, line2, line3).?;
        sum += priority(item);
    }
    std.debug.print("Part 2: {}\n", .{sum});
}
