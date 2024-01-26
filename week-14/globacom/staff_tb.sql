--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer,
    staff_name text,
    dno integer,
    staff_sal real,
    age integer,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
100	MUSTAPHA ALI	3	175000	32	08063285831
107	ALOKWE MARTIN	7	380000	48	07090082812
97	DANKADE AMINAT	5	550000	40	09023688832
108	JOSIAH JOSHUA	1	120000	30	08053189131
102	MANKINDE MARY	2	450000	55	09023487830
120	ADELEKE JANE	4	200000	38	07061045862
122	OSAHON MARK	6	320000	44	08022289842
104	KUTI LAWAL	1	750000	35	09145689842
117	SULEMAN AJAYI	3	800000	50	7030089981
\.


--
-- PostgreSQL database dump complete
--

