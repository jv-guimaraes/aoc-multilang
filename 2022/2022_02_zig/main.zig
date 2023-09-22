const std = @import("std");
const input = @embedFile("input.txt");

const Hand = enum {
    rock,
    paper,
    scissor,
};

pub fn new_hand(hand: []const u8) ?Hand {
    return switch (hand[0]) {
        'A', 'X' => Hand.rock,
        'B', 'Y' => Hand.paper,
        'C', 'Z' => Hand.scissor,
        else => null,
    };
}

fn hand_score(hand: Hand) u32 {
    return switch (hand) {
        Hand.rock => 1,
        Hand.paper => 2,
        Hand.scissor => 3,
    };
}

fn match_score(player: Hand, enemy: Hand) u32 {
    if (player == enemy) {
        return 3;
    }
    if (player == .rock and enemy == .paper) {
        return 0;
    }
    if (player == .paper and enemy == .scissor) {
        return 0;
    }
    if (player == .scissor and enemy == .rock) {
        return 0;
    }
    return 6;
}

fn player_wins(enemy: Hand) Hand {
    return switch (enemy) {
        .rock => .paper,
        .paper => .scissor,
        .scissor => .rock,
    };
}

fn player_loses(enemy: Hand) Hand {
    return switch (enemy) {
        .rock => .scissor,
        .paper => .rock,
        .scissor => .paper,
    };
}

fn calculate_score_1(game: []const u8) u32 {
    var tokens = std.mem.tokenize(u8, game, " ");
    const enemy = new_hand(tokens.next().?).?;
    const player = new_hand(tokens.next().?).?;
    return hand_score(player) + match_score(player, enemy);
}

fn calculate_score_2(game: []const u8) u32 {
    var tokens = std.mem.tokenize(u8, game, " ");
    const enemy = new_hand(tokens.next().?).?;
    const action = tokens.next().?;
    var player: Hand = undefined;
    if (action[0] == 'X') {
        player = player_loses(enemy);
    } else if (action[0] == 'Y') {
        player = enemy;
    } else {
        player = player_wins(enemy);
    }
    return hand_score(player) + match_score(player, enemy);
}

pub fn main() !void {
    // Part 1
    var lines = std.mem.tokenize(u8, input, "\r\n");
    var total: u32 = 0;
    while (lines.next()) |line| {
        total += calculate_score_1(line);
    }
    std.debug.print("Part 1: {}\n", .{total});

    // Part 2
    lines = std.mem.tokenize(u8, input, "\r\n");
    total = 0;
    while (lines.next()) |line| {
        total += calculate_score_2(line);
    }
    std.debug.print("Part 2: {}\n", .{total});
}
