# todo_actix
EZ todo application;

RESTAPI built with rust;

# Technology stacks :

-actix-web: backend framework(support http)

-mongodb: database

-built on clean architechture principle 
(uncle bob's architechture)

+Performance : not bad :) definitely faster than 
my previous built from nodejs(express-js)

+Development-time : 1 month; Done!

+Lazy to refactor some code/ add features since
this is just a educational project;

# Based on Clean architecture :

Caveats:

-This app get complicated very fast with all the 
abstraction that is needed to extract the business
logic from framework code;

-Time consuming / lots of planning

-If you don't plan on working in a long term project,
you would be benifited less to build project
in this architechture.(In my case)

Advantages:

-The code get very easy to debug and maintain. For
example, if there is any error in my domain logic,
I do not have to mess around or change any of my
actix-web code to fix which could unecessarily break
other component of my app.

-Once I finish the abstraction layer, intergrating  
the logic into http is very straighforward.

-Can easily change Database from mocks to real one
or switch to others.

My favorite advantage is the switchable database because
I could write my app without caring too much about it at first
and focus more on adding features and intergration.

Domain : http://localhost:8088/;
Port : 8080

# RESOURCES-ROUTES:
++ TODOLIST : 
get : http://localhost:8088/tasks;
get : http://localhost:8088/tasks/{id}/
post : http://localhost:8088/tasks;
delete : http://localhost:8088/tasks/{id}
update : http://localhost:8088/tasks/{id}
# Inside todolist there are tasks :
++ TASKS :
get : http://localhost:8088/todolist;
get : http://localhost:8088/todolist/{id}/
post : http://localhost:8088/todolist;
delete : http://localhost:8088/todolist/{id}
update : http://localhost:8088/todolist/{id}

# TODO: add in frontend 
# and docker support!
