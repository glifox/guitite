import { LoroDoc } from 'loro-crdt';
import { Conection } from './protocol/conection';
// import Bun from 'bun';

function pausecomp(millis: number) {
    var date: any = new Date();
    var curDate: any = null;
    do { curDate = new Date(); }
    while(curDate - date < millis);
}


const test = async () => {
  const doc = new LoroDoc();
  const doc2 = new LoroDoc();
  const connection = new Conection(doc, "ws://localhost:8080/ws/some");
  
  doc.getText("text").insert(0, "Hello");
  doc.commit;
  
  const connection2 = new Conection(doc2, "ws://localhost:8080/ws/some");
  
  pausecomp(2000);
  doc2.getText("text").insert(5, " World!");
  doc2.commit;
  
  
  pausecomp(2000);
  
  console.log("doc1:", doc.getText("text").toString())
  console.log("doc2:", doc2.getText("text").toString())
}


test()
  .catch()
  .then()
  .finally()
