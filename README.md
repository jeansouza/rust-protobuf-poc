# rust-protobuf-poc

docker run -p 15691:15691 -p 15692:15692 -p 25672:25672 -p 4369:4369 -p 5671:5671 -p 15672:15672 -p 5672:5672 rabbitmq


rabbitmq-plugins enable rabbitmq_management


rabbitmqctl add_user admin admin  


rabbitmqctl set_user_tags admin administrator
rabbitmqctl set_permissions -p / admin ".*" ".*" ".*"