
<?php
// Read input file
$lines = file("/usr/src/app/input.txt");

$depth = 0;
$horizontal = 0;
foreach ($lines as $line) {
  // Remove \n from line
  $line = rtrim($line);
  // Split line on whitespace
  $line = explode(' ', $line);
  switch ($line[0]) {
    case "forward":
      $horizontal += $line[1];
      break;
    case "down":
      $depth += $line[1];
      break;
    case "up":
      $depth -= $line[1];
      break;
  }
}

// echo "horizontal " . $horizontal . "\n";
// echo "depth " . $depth . "\n";
echo "result " . ($horizontal * $depth) . "\n";
?>
