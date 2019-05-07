#!/usr/bin/perl -w

use strict;

while(1) {

my $sign = int(rand 2);
my $exc = int(rand 3);
my $len = int(rand 9);

my $x = (rand 1);

$x = int($x * (10 ** $len)) / 10 ** $len;

if ($exc == 0) {
    # small exponent
    $x *= 10 ** ( int(rand 8) - 3);
} elsif ($exc == 1) {
    # medium
    $x *= 10 ** ( int(rand 80) - 30);
} else {
    $x *= 10 ** ( int(rand 1000) - 500);
}

if ($sign) {
    $x = -$x;
}

print "$x\n";



}
