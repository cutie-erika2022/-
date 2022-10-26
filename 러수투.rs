fn fo(_a:i32)
{
    for i in 1.._a+1
    {
        print!("{}\n",i);
    }
}

fn main()
{
    fo(7000000);
}
