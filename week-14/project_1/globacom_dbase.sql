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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer,
    dno integer,
    dname text,
    dlocation text,
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer,
    pname text,
    pduration text,
    project_managerid integer
);


ALTER TABLE public.project OWNER TO postgres;

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
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	ADMINISTRATION	IKEJA	44
101	2	ACCOUNT	EGBEDA	11
100	3	PACKAGING	AJAH	44
120	4	RESEARCH	V.I	33
97	5	ACCOUNT	MAGODO	22
122	6	OPERATIONS	MILE 2	44
107	7	PACKAGING	KETU	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 MONTHS	102
22	B	14 MONTHS	97
33	C	16 MONTHS	120
44	D	25 MONTHS	108
55	E	9 MONTHS	107
\.


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

