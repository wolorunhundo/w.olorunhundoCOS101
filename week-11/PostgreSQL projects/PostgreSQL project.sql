--  Create and connect to the database
CREATE DATABASE mtnnigeria_db;
\c mtnnigeria_db;

--  Create the EMPLOYEES table
CREATE TABLE employees (
    eid INT PRIMARY KEY NOT NULL,
    ename TEXT NOT NULL,
    dno INT NOT NULL,
    esal REAL NOT NULL,
    age INT NOT NULL,
    phone VARCHAR(20) NOT NULL
);

--  Create the DEPARTMENT table
CREATE TABLE department (
    dept_managerid INT NOT NULL,
    dno INT PRIMARY KEY NOT NULL,
    dname TEXT NOT NULL,
    dlocation TEXT NOT NULL,
    pno INT NOT NULL
);

--  Create the PROJECT table
CREATE TABLE project (
    pno INT PRIMARY KEY NOT NULL,
    pname TEXT NOT NULL,
    pduration TEXT NOT NULL,
    project_managerid INT NOT NULL
);

--  Insert records into EMPLOYEES
INSERT INTO employees (eid, ename, dno, esal, age, phone) VALUES
(101, 'Alade Joy', 2, 250000.00, 33, '08023089832'),
(100, 'Mustapha Ali', 3, 175000.00, 32, '08063285831'),
(107, 'Alokwe Martin', 7, 380000.00, 48, '07090082812'),
(97, 'Dankade Aminat', 5, 550000.00, 40, '09023688832'),
(108, 'Josiah Joshua', 1, 120000.00, 30, '08053189131'),
(102, 'Mankinde Mary', 2, 450000.00, 55, '09023487830'),
(120, 'Adeleke Jane', 4, 200000.00, 38, '07061045862'),
(122, 'Osahon Mark', 6, 320000.00, 44, '08022289842'),
(117, 'Suleman Ajayi', 3, 800000.00, 50, '07030089811'),
(104, 'Kuti Lawal', 1, 750000.00, 35, '09145689842');

-- Insert records into DEPARTMENT
INSERT INTO department (dept_managerid, dno, dname, dlocation, pno) VALUES
(108, 1, 'Administration', 'Ikeja', 4),
(101, 2, 'Account', 'Egbeda', 1),
(100, 3, 'Packaging', 'Ajah', 4),
(97, 4, 'Research', 'V.I.', 3),
(122, 5, 'Account', 'Magodo', 2),
(107, 6, 'Operations', 'Mile 2', 4),
(7, 7, 'Packaging', 'Ketu', 5);

-- Insert records into PROJECT
INSERT INTO project (pno, pname, pduration, project_managerid) VALUES
(1, 'A', '9 Months', 102),
(2, 'B', '14 Months', 97),
(3, 'C', '16 Months', 120),
(4, 'D', '25 Months', 108),
(5, 'E', '9 Months', 107);

--  To see tables and data
\d
SELECT * FROM employees;
SELECT * FROM department;
SELECT * FROM project;