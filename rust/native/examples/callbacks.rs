use virtblocks_rust_native::ultra::Ultra;

fn main() {
    let mut u = Ultra::default();
    let cb = |x| println!("{}", x);

    u.set_cb(cb);

    u.call_me();
    u.call_me();

    u.unset_cb();

    u.call_me();
    u.call_me();
    u.call_me();

    u.set_cb(cb);

    u.call_me();
    u.call_me();
}
