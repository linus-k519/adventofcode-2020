
<?php
// Read input file
$lines = file("input.txt");

$depth = 0;
$horizontal = 0;
$aim = 0;
foreach ($lines as $line) {
  // Remove \n from line
  $line = rtrim($line);
  // Split line on whitespace
  $line = explode(' ', $line);
  switch ($line[0]) {
    case "forward":
      $horizontal += $line[1];
      $depth += $aim * $line[1];
      break;
    case "down":
      $aim += $line[1];
      break;
    case "up":
      $aim -= $line[1];
      break;
  }
}

// echo "horizontal " . $horizontal . "\n";
// echo "depth " . $depth . "\n";
echo "result " . ($horizontal * $depth) . "\n";
?>
