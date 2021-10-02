#!/usr/bin/perl -w

use strict;


my $REPL = "./target/debug/examples/repl";
$REPL=$ENV{"REPL"} if exists $ENV{"REPL"};

if (! -e $REPL) {
    die "$REPL does not exist"
}

open N, "<", "numbers.txt";

my @A;

while(<N>) {
    chomp;
    push @A, $_;
}

close N;


open R, ">", "src/test.rs";
print R "use super::PrettyPrintFloat as P;\n";
print R "fn p(x:&'static str) -> f64 { x.parse().unwrap() }\n";


our @names = ("default_settings", "one", "onetwo", "two", "onethree", "twothree", "three", "onefour", "twofour", "threefour", "four", "five", "six", "many1", "many2");
our @strs =  ("",                ":1.1", ":1.2"  ,":2.2", ":1.3"    , ":2.3"    , ":3.3" , ":1.4"   , ":2.4"   ,  ":3.4"    , ":4.4",":5.5" ,":6.6" ,":30.30",":20.30");


for my $suite (0 .. $#names) {

    my $name = $names[$suite];
    my $str = $strs[$suite];

    print "$name $str\n";

    my $chunk_counter = 0;
    my $remaining_in_this_chunk = 0;
    
    print R "fn _just_opening_brace_$name() {\n";

    foreach (@A) {
        chomp;
        my $x = $_;
        open F, ">", "in.txt";
        if ($str) {
            $str =~ m@:(\d+)\.(\d+)@;
            print F "width=$1\n";
            print F "prec=$2\n";
        }
        print F "$x";
        close F;

        open E, "cat in.txt | $REPL 2> /dev/null|";
        my $out = join '', <E>;
        close E;
        chomp $out;

        #print "$x $out\n";
        print ".";

        if ($remaining_in_this_chunk == 0) {
            $chunk_counter += 1;
            $remaining_in_this_chunk = 30;
            print R "}\n";
            print R "#[test]\n";
            print R "fn ${name}_$chunk_counter() {\n";
        }
        $remaining_in_this_chunk -= 1;

        if ($str =~ /^\:(\d+)\.(\d+)$/) {
            my $minl = $1;
            my $maxl = $2;
            my $samplel = length($out);
            #print STDERR "minl=$minl maxl=$maxl samplel=$samplel\n";
            if ($samplel > $maxl or $samplel < $minl) {
                print STDERR "\nError: generated sample \"$out\" for number $x with format $str has length ($samplel) not between $minl and $maxl\n";
                exit 1;
            }
        } else {
        }

        print R "    assert_eq!(format!(\"{$str}\", P(p(\"$x\"))), \"$out\");\n";

    }
    print R "}\n";
    print "\n";

}

close R;
