use strict;
use warnings;
use JSON qw( decode_json );
use Data::Dumper;
use List::Util qw( any );

my @data = ();

my %fnMap = (
    TOKENS         => \&TOKENS,
    TOKEN_FROM_STR => \&TOKEN_FROM_STR
);

sub main {
    # Read grammar.json file, assign globally
    read_data();

    # Generate Token enum from grammar.json file, match names to token types
    print "Reading template files... ";
    open TEMPLATE , "<" , "./templates/tokens.rs.template" or die "Can't open tokens.rs.template: $!";
    open OUT , ">" , "./src/gen/tokens.rs" or die "Can't open tokens.rs: $!";
    my @lines = ();
    push @lines, $_ while <TEMPLATE>;
    my $template = join '', @lines;
    close TEMPLATE;
    print "Done.\n";

    # Find all template inserts and replace them with the output from fnMap
    while ($template =~ /\$<(\w+)>/g) {
        print "Generating $1... ";
        my $fn = $1;
        next unless any { $_ eq $fn } keys %fnMap;
        my $out = $fnMap{$fn}->();
        $template =~ s/\$<$fn>/$out/g;
        print "Done.\n";
    }

    # Write the output to tokens.rs
    print "Writing tokens.rs... ";
    print OUT $template;
    print "Done.\n";
    close OUT;
}

sub read_data() {
    open GRAMMAR , "<" , "./src/strawberry/grammar.json" or die "Can't open grammar.json: $!";
    my @lines = ();
    push @lines, $_ while <GRAMMAR>;
    close GRAMMAR;

    my $data = decode_json(join "", @lines);
    @data = @{$data};
}

sub TOKENS {
    my $out = '';
    for my $rule (@data) {
        next if ($rule->{method} eq "Ignore");
        my $name = $rule->{name};
        $out .= "\t$name(String),\n";
    }
    return $out;
}

sub TOKEN_FROM_STR {
    my $out = '';
    for my $rule (@data) {
        next if ($rule->{method} eq "Ignore");
        my $name = $rule->{name};
        $out .= "\t\t\t\"$name\" => Token::$name(value),\n";
    }
    return $out;
}

main();
