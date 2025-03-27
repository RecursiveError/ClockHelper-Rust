const std = @import("std");

pub fn main() !void {
    var arena_state = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena_state.deinit();
    const arena = arena_state.allocator();

    const cwd = std.fs.cwd();
    cwd.deleteFile("src/ClockTrees/mod.rs") catch {};
    var main_file = try cwd.createFile("src/ClockTrees/mod.rs", .{});

    var tree_dir = try cwd.openDir("src/ClockTrees", .{ .iterate = true });
    var iter = tree_dir.iterate();
    while (true) {
        const file = try iter.next();
        if (file) |entry| {
            switch (entry.kind) {
                .file => {
                    const name = entry.name;
                    if (std.mem.eql(u8, name, "mod.rs")) continue;
                    if (std.mem.indexOf(u8, name, ".rs")) |_| {
                        const short_name = name[0..(name.len - 3)];
                        try main_file.writer().print(
                            \\#[cfg(feature = "{s}")]
                            \\pub mod {s};
                            \\
                        , .{ short_name, short_name });
                        try std.io.getStdOut().writer().print("{s} = []\n", .{short_name});
                    }
                },
                else => {},
            }
            continue;
        }
        break;
    }

    main_file.close();
    var ch = std.process.Child.init(&[_][]const u8{ "cargo", "fmt", "--all" }, arena);
    _ = try ch.spawnAndWait();
}
