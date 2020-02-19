<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Coordinate Emergency Room              _750916</name>
   <tag></tag>
   <elementGuidId>5e6a7d3a-4071-4ef5-a4c7-decafb1adfca</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='triageContent']/div/div/div/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>col</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                            
                                
                                    Coordinate: Emergency Room
                                
                                
                                    
                                
                            
                                
                                    
                                        LOCAL EMERGENCY # 911
                                    
                                    
                                
                            
                                





    
        
            
                
                    Hospital And Shelter D'youville
                        
                
                    
                        
                    
            
                
            
                
                
                    1036 Rue Belvédère S, Sherbrooke, QC J1H 4C4, Canada , Sherbrooke, Québec, J1H 4C4  
                    Phone: (819) 780-2222 - Fax:   
                        Hours: Thursday: Open 24 hours  
                    Distance: 5.04 Km / 3.13 Mi
                    Travel Time: 9 mins
                
            
        
        
        
    
    
        
            
                
                    Hôtel-Dieu De Sherbrooke (chus)
                        
                
                    
                        
                    
            
                
            
                
                
                    580 Rue Bowen Sud , Sherbrooke, Québec, J1G 2K8  
                    Phone: (819) 346-1110 - Fax:   
                        Hours: Thursday: Open 24 hours  
                    Distance: 5.70 Km / 3.53 Mi
                    Travel Time: 9 mins
                
            
        
        
        
    
    
        
            
                
                    Hospital À Sherbrooke
                        
                
                    
                        
                    
            
                
            
                
                
                    580 Rue Bowen S, Sherbrooke, QC J1G 2K8, Canada , Sherbrooke, Québec, J1G 2K8  
                    Phone: (819) 346-1110 - Fax:   
                        Hours: Thursday: Open 24 hours  
                    Distance: 5.70 Km / 3.53 Mi
                    Travel Time: 9 mins
                
            
        
        
        
    
    
        
            
                
                    Hôpital Fleurimont (chus)
                        
                
                    
                        
                    
            
                
            
                
                
                    3001 12e Avenue Nord , Sherbrooke, Québec, J1H 5H3  
                    Phone: (819) 346-1110 - Fax:   
                        Hours: Thursday: Open 24 hours  
                    Distance: 12.64 Km / 7.84 Mi
                    Travel Time: 18 mins
                
            
        
        
        
    
    
        
            
                
                    Centre Hospitalier Universitaire De Sherbrooke (université De Sherbrooke)
                        
                
                    
                        
                    
            
                
            
                
                
                    3001 12e Avenue Nord , Sherbrooke, Québec, J1H 5H3  
                    Phone: (819) 821-8000 - Fax:   
                        Hours: Thursday: Open 24 hours  
                    Distance: 12.64 Km / 7.84 Mi
                    Travel Time: 18 mins
                
            
        
        
        
    
    
        
            
                
                    Hospital Center And Accommodation Memphrémagog
                        
                
                    
                        
                    
            
                
            
                
                
                    33 Rue Saint-Patrice E, Magog, QC J1X 1T5, Canada , Magog, Québec, J1X 1T5  
                    Phone: (819) 843-2572 - Fax:   
                        Hours: Unknown  
                    Distance: 32.93 Km / 20.42 Mi
                    Travel Time: 29 mins
                
            
        
        
        
    


            
                Hospital And Shelter D'youvilleFacility Hours:Sunday: Open 24 hoursMonday: Open 24 hoursTuesday: Open 24 hoursWednesday: Open 24 hoursThursday: Open 24 hoursFriday: Open 24 hoursSaturday: Open 24 hours
            
            
                Hotel-Dieu De Sherbrooke (chus)Facility Hours:Sunday: Open 24 hoursMonday: Open 24 hoursTuesday: Open 24 hoursWednesday: Open 24 hoursThursday: Open 24 hoursFriday: Open 24 hoursSaturday: Open 24 hours
            
            
                Hospital In SherbrookeFacility Hours:Sunday: Open 24 hoursMonday: Open 24 hoursTuesday: Open 24 hoursWednesday: Open 24 hoursThursday: Open 24 hoursFriday: Open 24 hoursSaturday: Open 24 hours
            
            
                Fleurimont Hospital (chus)Facility Hours:Sunday: Open 24 hoursMonday: Open 24 hoursTuesday: Open 24 hoursWednesday: Open 24 hoursThursday: Open 24 hoursFriday: Open 24 hoursSaturday: Open 24 hours
            
            
                Sherbrooke University Hospital Centre (University of Sherbrooke)Facility Hours:Sunday: Open 24 hoursMonday: Open 24 hoursTuesday: Open 24 hoursWednesday: Open 24 hoursThursday: Open 24 hoursFriday: Open 24 hoursSaturday: Open 24 hours
            
            
                Hospital Center And Accommodation Memphremagog
            



    var flagFacility = 'Flag Facility';
    $(document).ready(function() {
        if ($('#ShowingClosed').val() == 1) {
            $('#chkShowClosed').prop('checked', true);
        }

        $('.tooltipCoordClass').tooltipster({
            theme: 'tooltipster-noir',
            side: 'right'
        });

        $('#flagFacility').tooltipster({
            theme: 'tooltipster-noir'
        });
        
    });


                                
                                    Map DataMap data ©2020 GoogleMap DataMap data ©2020 GoogleMap data ©2020 GoogleTerms of UseReport a map errorMapTerrainSatelliteLabels

                                    
                        function initAutocomplete() {


                            var map = new google.maps.Map(document.getElementById('map'),
                                {
                                    center: { lat: 45.3636791, lng: -71.8566038 },
                                    zoom: 15,
                                    mapTypeId: 'roadmap'
                                });
                            //Create the search box and link it to the UI element.
                            var input = document.getElementById('pac-input');


                            var searchBox = new google.maps.places.SearchBox(input);
                            //map.controls[google.maps.ControlPosition.TOP_LEFT].push(input);

                            //Bias the SearchBox results towards current map's viewport.
                            map.addListener('bounds_changed',
                                function() {
                                    searchBox.setBounds(map.getBounds());
                                });

                            var markers = [];
                            //Listen for the event fired when the user selects a prediction and retrieve
                            //more details for that place.
                            searchBox.addListener('places_changed',
                                function() {
                                    var places = searchBox.getPlaces();

                                    if (places.length == 0) {
                                        return;
                                    }

                                    // Clear out the old markers.
                                    markers.forEach(function(marker) {
                                        marker.setMap(null);
                                    });
                                    markers = [];
                                    debugger;
                                    // For each place, get the icon, name and location.
                                    var bounds = new google.maps.LatLngBounds();
                                    places.forEach(function(place) {
                                        if (!place.geometry) {
                                            console.log(&quot;Returned place contains no geometry&quot;);
                                            return;
                                        }
                                        var icon = {
                                            url: place.icon,
                                            size: new google.maps.Size(71, 71),
                                            origin: new google.maps.Point(0, 0),
                                            anchor: new google.maps.Point(17, 34),
                                            scaledSize: new google.maps.Size(25, 25)
                                        };

                                        // Create a marker for each place.
                                        markers.push(new google.maps.Marker({
                                            map: map,
                                            icon: icon,
                                            title: place.name,
                                            position: place.geometry.location
                                        }));

                                        if (place.geometry.viewport) {
                                            // Only geocodes have viewport.
                                            bounds.union(place.geometry.viewport);
                                        } else {
                                            bounds.extend(place.geometry.location);
                                        }
                                    });
                                    map.fitBounds(bounds);
                                });
                        }

                                    
                                    
                                

                            
                            
                                


                                        ScoreDistance
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                        
                                                    
                                                
                                                73 Queen, Sherbrooke, Quebec J1M 0C9, Canada
                                            
                                        
                                    

                                    
                                        
                                            
                                                
                                                    
                                                
                                                
                                                    
                                                        
                                                    
                                                    
                                                
                                            
                                        

                                    
                                

                            
                        </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;triageContent&quot;)/div[@class=&quot;row mainTriageRow  mh-100&quot;]/div[@class=&quot;col-12&quot;]/div[@class=&quot;row  mh-100&quot;]/div[@class=&quot;col&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='triageContent']/div/div/div/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Phone'])[1]/following::div[10]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Name'])[1]/following::div[11]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div[2]/div/div/div/div</value>
   </webElementXpaths>
</WebElementEntity>
