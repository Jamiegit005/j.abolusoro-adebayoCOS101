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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer,
    c_name text,
    c_age integer,
    c_email text,
    c_mobile text,
    eid integer,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	MUSTA KARIM	35	M_KARIM@GMAIL.COM)	08055089112	102	5
11	LILIAN JAIYA	43	I_JAIYE@GMAIL.COM	08055185341	100	3
112	ARTHUR MUSA	50	A_MUSA@GMAIL.COM	07055282813	107	10
113	PHILIP AKONIO	41	P_AKONJO@GMAIL.COM	09052356772	100	2
114	MARYLENE MAPA	33	M_MAPA@GMAIL.COM	08053333551	120	5
115	OGHENERO AGOR	50	O_AGOR@GMAIL.COM	07055566774	117	11
116	ADAMS BREE	33	A_BREE@GMAIL.COM	08056765424	102	1
117	OKAFOR MATHIAS	45	O_MATHIAS@GMAIL.COM	08056763367	120	10
118	SAMSON ADELEKE	65	S_ADELEKE@GMAIL.COM	07056774423	117	11
119	LAWAL TAMIRE	35	L_TAMIRE@GMAIL.COM	09052111101	107	5
120	JAMES JOB	44	J_JOB@GMAIL.COM	08059693919	100	8
121	MATTHEW JAKANDE	21	M_JAKANDE@GMAIL.COM	07051232144	120	2
122	JIMILA ADEGBOYE	20	J_ADEGBOYE@GMAIL.COM	08054921923	107	5
\.


--
-- PostgreSQL database dump complete
--

