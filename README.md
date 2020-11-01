# todo_actix
EZ todo application;
RESTAPI built with rust;

Technology stacks :
-actix-web: backend framework(support http)
-mongodb: database
-built on clean architechture principle 

+Performance : not bad :) definitely faster than 
my previous built from nodejs(express-js)

+Development-time : 1 month; Done!
+Lazy to refactor some code/ add features since
this is just a educational project;
+As a noob, I got stuck a lot of time trying to
reason the architechture at first, but there are
good resources on the internet to help you out.


Based on Clean architecture :
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
actix-web code to fix it.
-Very testable; can test business logic in isolation 
from the framework.
-Once I finish the abstraction layer, intergrating  
the logic into http is very straighforward.
-Can easily change Database from mocks to real one
or switch to others.


TODO: add in frontend and docker support!
