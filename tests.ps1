$error_name = "err";
$description = "This is a test"

$current_date = (Get-Date).DateTime
.\target\debug\udoc.exe new -n $error_name -d $description

