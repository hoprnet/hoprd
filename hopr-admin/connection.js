/*
 * Maintain a websocket connection
 */

const MAX_MESSAGES_CACHED = 50

export class Connection {
  logs = []

  constructor(
    setConnecting,
    setMessages,
    setConnectedPeers
  ){
    this.setConnecting = setConnecting
    this.setMessages = setMessages
    this.setConnectedPeers = setConnectedPeers
    this.connect()
  }

  appendMessage(event) {
    try {
      const msg = JSON.parse(event.data)
      if (msg.type == 'log') {
        if (this.logs.length > MAX_MESSAGES_CACHED){ // Avoid memory leak
          this.logs.splice(0, this.logs.length - MAX_MESSAGES_CACHED); // delete elements from start
        }
        this.logs.push(msg)
        this.setMessages(this.logs.slice(0)) // Need a clone
      } else if (msg.type == 'connected'){
        this.setConnectedPeers(msg.msg.split(','))
      }
    } catch (e) {
      console.log("ERR", e)
    }
  }

  sendMessage(message){
    if (!this.client) {
      console.error('No client to send')
      return
    }
    this.client.send(message)
  }

  connect() {
    console.log('Connecting ...')
    var client = this.client = new WebSocket('ws://' + window.location.host);
    console.log('Web socket created')

    client.onopen = () => {
      console.log('Web socket opened')
      this.setConnecting(false)
    }

    client.onmessage = (event) => {
      this.appendMessage(event)
      console.log(event)
    }

    client.onerror = (error) => {
      console.log('Connection error:', error)
    }

    client.onclose = () => {
      console.log('Web socket closed')
      delete this.client
      this.setConnecting(true)
      this.appendMessage(' --- < Lost Connection, attempting to reconnect... > ---')
      setTimeout(function(){
        try {
          connect()
          console.log('connection')
        } catch (e){
          console.log('Error connecting', e)
        }
      }, 1000);
    }
  }

  disconnect(){
    if (this.client) {
      this.client.close()
    }
  }
}
