create or replace NONEDITIONABLE procedure ss_log (
  p_message in varchar2
) is
  l_status integer;
begin
  dbms_pipe.pack_message('[' || sysdate || '] ' || p_message);
  l_status := dbms_pipe.send_message('log');
end ss_log;
/

declare
  l_status  integer;
  l_message varchar2(4000 char);
begin
  while true loop
    l_status := dbms_pipe.receive_message('log');
    if l_status = 0 then
      dbms_pipe.unpack_message(l_message/*out*/);
      dbms_output.put_line(l_message);
    end if;
  end loop;
end;
/
