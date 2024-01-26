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
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer,
    data_size text,
    data_duration integer,
    data_price text
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200NAIRA
2	1.8GB	14	500NAIRA
3	3.9GB	30	1000NAIRA
4	7.5GB	30	1500NAIRA
5	9.2GB	30	2000NAIRA
6	10.8GB	30	2500NAIRA
7	14GB	30	3000NAIRA
8	18GB	30	4000NAIRA
9	24GB	30	5000NAIRA
10	29.9GB	30	8000NAIRA
11	50GB	30	10000NAIRA
\.


--
-- PostgreSQL database dump complete
--

