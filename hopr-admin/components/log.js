import React, { useEffect, useState, useRef } from "react";
import styles from '../styles/log.module.css'
import dynamic from "next/dynamic";

const Jazzicon = dynamic(() => import("../components/jazzicon"), { ssr: false });

const ID_REGEX = /(\w{53})/g

export function LogLine(props){
  let raw = props.value.msg
  let msg = []
  let ids = []
  let match

  let lastIndex = 0
  while ((match = ID_REGEX.exec(raw)) !== null){
    ids.push(match[0])
    msg.push(match.input.slice(lastIndex, match.index))
    msg.push(<abbr title={match[0]}>{match[0].slice(48)}</abbr>)
    lastIndex = match.index + match[0].length
  }
  if (msg.length == 0) {
    msg = raw // No matches
  }

  return (
    <div key={props.value.ts} className={styles.logline}>
      <time>{ props.value.ts.slice(11) }</time>
      <pre>{ msg }</pre>
      <div className={styles.loglineicons}>
        {ids.map(x => 
                <Jazzicon
                  key={x}
                  diameter={15}
                  address={x}
                  />
              )
        }
        &nbsp;
      </div>
    </div>
  )
}


let prevLog = ''

export function Logs({
  connecting,
  messages,
  connection
}){
  let container = useRef(null)

  useEffect(() => {
    container.current.scrollIntoView({block: 'end', behaviour: 'smooth'});
  })


  let onKeyDown = (e) => {
    if (e.keyCode == 13 ) { // enter 
      var text = e.target.value 
      console.log("Command: ", text)
      if (connection && text.length > 0) {
        connection.sendMessage(text)
        prevLog = text
        e.target.value = ""
      }
    }
    if (e.keyCode == 38) { // Up Arrow
      e.target.value = prevLog
    }
  }

  let cls = styles.logs + ' ' + (connecting ? styles.connecting : '')
  return (
    <div>
      <div className={cls}>
        <div ref={container}>
        { messages.map(x => <LogLine value={x} />) }
        </div>
      </div>
      <div className={styles.send}>
        <input id="command"
          type="text"
          disabled={connecting}
          onKeyDown={onKeyDown}
          autoFocus
          placeholder="type 'help' for full list of commands" /> 
      </div>
    </div>
  )
}
