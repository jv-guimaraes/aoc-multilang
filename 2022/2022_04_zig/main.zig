const std = @import("std");
const input = @embedFile("input.txt");
const parseInt = std.fmt.parseInt;
const tokenize = std.mem.tokenize;

const Range = struct {
    left: usize,
    right: usize,

    fn contains(self: Range, other: Range) bool {
        if (self.left <= other.left and self.right >= other.right) {
            return true;
        }
        return false;
    }

    fn new(left: usize, right: usize) Range {
        return Range{ .left = left, .right = right };
    }

    fn from_string(string: []const u8) Range {
        var tokens = tokenize(u8, string, "-");
        const left: usize = parseInt(usize, tokens.next().?, 10) catch unreachable;
        const right: usize = parseInt(usize, tokens.next().?, 10) catch unreachable;
        return Range.new(left, right);
    }

    fn overlap(self: Range, other: Range) bool {
        if (self.left >= other.left and self.left <= other.right) {
            return true;
        }
        if (self.right >= other.left and self.right <= other.right) {
            return true;
        }
        return false;
    }
};

pub fn main() !void {
    // Part 1
    var lines = tokenize(u8, input, "\r\n");
    var count: usize = 0;
    while (lines.next()) |line| {
        var split = tokenize(u8, line, ",");
        const r1 = Range.from_string(split.next().?);
        const r2 = Range.from_string(split.next().?);
        if (r1.contains(r2) or r2.contains(r1)) {
            count += 1;
        }
    }
    std.debug.print("Part 1: {}\n", .{count});

    // Part 2
    lines.reset();
    count = 0;
    while (lines.next()) |line| {
        var split = tokenize(u8, line, ",");
        const r1 = Range.from_string(split.next().?);
        const r2 = Range.from_string(split.next().?);
        if (r1.overlap(r2) or r2.overlap(r1)) {
            count += 1;
        }
    }
    std.debug.print("Part 2: {}\n", .{count});
}
