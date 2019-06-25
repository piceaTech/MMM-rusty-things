

Module.register("rusty-things",{
  // Default module config.
  defaults: {
    
  },
  todayEntries: [],
  tomorrowEntries: [],
  inboxEntries: [],
  currentState: 0,


  getTemplate: function () {
    return `template.njk`;
  },

  getTemplateData: function () {
    return {
      config: this.config,
      count: {
        today: this.todayEntries.length,
        tomorrow: this.tomorrowEntries.length,
        inbox: this.inboxEntries.length,
      },
      todayEntries: this.todayEntries.slice(0,20),
    }
  },
  start: function(){
    const self = this;
    self.sendSocketNotification('REQUEST_ENTRIES')
    self.sendSocketNotification('CLIENT_ACTIVE', {currentState: this.currentState});
    setInterval(function() {
      self.sendSocketNotification('CLIENT_ACTIVE', {currentState: self.currentState})
    }, 20 * 1000); //perform every 20 secs.
  },
  socketNotificationReceived: function(notification, payload){
    console.log('client:', notification, payload);
    if(notification === "RESPONSE_ENTRIES"){
      this.todayEntries = payload.today;
      this.tomorrowEntries = payload.tomorrow;
      this.inboxEntries = payload.inbox;
      this.currentState = payload.currentState;
      this.updateDom();
    }
    
    if(notification === "DB_UPDATED"){
      this.sendSocketNotification('REQUEST_ENTRIES')
    }
    
    if(notification === "RELOAD"){
      window.location.reload(); 
    }
  }
});