--
-- PostgreSQL database dump
--

\restrict K6uw23Zj5I3SyeqKGflovtU5a00l5AsmQaDQ6iyfVd8mjLLcQTrjENiMDJnfqah

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	Victoria Island	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dno);


--
-- PostgreSQL database dump complete
--

\unrestrict K6uw23Zj5I3SyeqKGflovtU5a00l5AsmQaDQ6iyfVd8mjLLcQTrjENiMDJnfqah

