my $acc = 0;
while(<>) {
    $acc += length($_) - 1;
    print;
}
print $acc . "\n";

