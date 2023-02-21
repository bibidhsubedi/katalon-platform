<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_WE DO NOT SELL NICOTINE AS IT IS ILLEGA_8c27a9</name>
   <tag></tag>
   <elementGuidId>50f4abd9-f93b-4264-945f-2f20681aab30</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.overflow-hidden</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Yes'])[1]/following::div[1]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>9e222988-b376-4464-900a-ed6f379e90a3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>overflow-hidden</value>
      <webElementGuid>aa151523-9882-4717-8ce4-ffd57e2127dc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
      
        
          
        
      
      
        
          
        
      
      
        
          
            
            
            
          
        
        
          
            
              
              
            
          
        

        
  
    
      
        
          
        
        
          
            
              
              
            
          
        
        
          
        
       
      
        
  
  
  
    
      
      
    
  

      
    
    
       
          WE DO NOT SELL NICOTINE AS IT IS ILLEGAL TO SELL NICOTINE IN AUSTRALIA
      
    
    
      
        
          
            
              
                
                  
                    
                      
                      
                    
                  
                
              
              
            
          
          
            
              
                
                  
                    
  
  
  
    
      
      
    
  

                    
  $(function() {
    // Current Ajax request.
    var currentAjaxRequest = null;
    // Grabbing all search forms on the page, and adding a .search-results list to each.
    var searchForms = $('form[action=&quot;/search&quot;]').css('position','relative').each(function() {
      // Grabbing text input.
      var input = $(this).find('input[name=&quot;q&quot;]');
      // Adding a list for showing search results.
      if ($('.template-search').length > 0) {
        var offSet = input.position().top + input.innerHeight() + 50;
      } else {
        if ($(window).width() > 1199) {
        var offSet = input.position().top + input.innerHeight() + 177;
        } else {
          var offSet = input.position().top + input.innerHeight() + 107;
        }
      }
      $('&lt;ul class=&quot;search-results has-scroll row&quot;>&lt;/ul>').appendTo($(this)).wrap('&lt;div class=&quot;block_search_autocomplete&quot;>&lt;/div>');
      $('.block_search_autocomplete').css( { 'position': 'absolute', 'left': '0px', 'top': '71px', 'z-index': '99' } ).hide();
      $('.search_trend').hide();
      // Listening to keyup and change on the text field within these search forms.
      input.attr('autocomplete', 'off').bind('keyup change', function() {
        // What's the search term?
        var term = $(this).val();
        // What's the search form?
        var form = $(this).closest('form');
        // What's the search URL?
        var searchURL = '/search?type=product&amp;q=' + term;
        // What's the search results list?
        var resultsList = form.find('.search-results');
        // If that's a new term and it contains at least 3 characters.
        if (term.length > 3 &amp;&amp; term != $(this).attr('data-old-term')) {
          // Saving old query.
          $(this).attr('data-old-term', term);
          // Killing any Ajax request that's currently being processed.
          if (currentAjaxRequest != null) currentAjaxRequest.abort();
          // Pulling results.
          currentAjaxRequest = $.getJSON(searchURL + '&amp;view=json', function(data) {
            // Reset results.
            resultsList.empty();
            // If we have no results.
            if(data.results_count == 0) {
              // resultsList.html('&lt;li>&lt;span class=&quot;title&quot;>No results.&lt;/span>&lt;/li>');
              // resultsList.fadeIn(200);
              $('.block_search_autocomplete').hide();
              $('.search_trend').hide();
            } else {
              // If we have results.
              $.each(data.results, function(index, item) {
                var link = $('&lt;a>&lt;/a>').attr('href', item.url);
                link.append('&lt;div class=&quot;thumbnail&quot;>&lt;img src=&quot;' + item.thumbnail + '&quot; />&lt;/div>');
                link.append('&lt;div class=&quot;media-body&quot;>&lt;div class=&quot;title&quot;>' + item.title + '&lt;/div>&lt;div class=&quot;price&quot;>' + item.price + '&lt;/div>&lt;/div>');
                //link.append('&lt;div class=&quot;price&quot;>' + item.price + '&lt;/div>');
                link.wrap('&lt;li class=&quot;col-lg-cus-5 col-md-cus-5&quot;>&lt;/li>');
                resultsList.append(link.parent());
              });
              // The Ajax request will return at the most 10 results.
              // If there are more than 10, let's link to the search results page.
              if(data.results_count > 4) {
                resultsList.append('&lt;li>&lt;a class=&quot;see_all&quot; href=&quot;' + searchURL + '&quot;>See all results (' + data.results_count + ')&lt;/a>&lt;/li>');
              }
              $('.block_search_autocomplete').fadeIn(200);
              $('.search_trend').show();
            }        
          });
        }
      });
    });
    // Clicking outside makes the results disappear.
    $('body').bind('click', function(){
      $('.block_search_autocomplete').hide();
      $('.search_trend').hide();
    });
  });

                  
                
              
            
            
              
            
            
              
                
                
                  
                    
                      
                      
                        
                      
                    
                  
                
              
            
          
        
      
    
    
      
        
          
          
            
  
    
    
    


    
      
    

    

    

      
        
          
            60ml VS E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            100ml VS E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            INTERSTATE &amp; INTERNATIONAL E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            COILS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            KITS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            MODS 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            TANKS 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            BATTERIES &amp; CHARGERS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            REPLACEMENT GLASS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            DIY
          
          
            
              
                
                  
                    
                      DIY PRODUCTS
                    
                  
                    
                      DIY PREMIUM CONCENTRATES
                    
                  
                
              
            
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            VS STANDARD CONCENTRATES 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            STORE LOCATOR
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            BLOGS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            Nicotine Script
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            ADULT PRODUCTS
          
        
      

    
  


          
          
  
    

    

    

    
    
    

    

    

    

    

    

    
  

        
      
    
  
  



        
          
            


  
    
      
        COILS
      
      
        
          
            Home
          
          
        
        
          
            
              COILS
              
            
          
        
      
      
      
    
  


  #NovBreadcrumbs::before {
    content: '';
    display: block;
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    background: rgba(0, 0, 0, 50%);
  }
  
  #NovBreadcrumbs {
    padding-top: 145px;
    padding-bottom: 126px;
    
    
    background-image: url(&quot;//cdn.shopify.com/s/files/1/2663/3040/files/breadcrumb-background_1920x.jpg?v=1651648211&quot;);
    background-repeat: no-repeat;
    background-position: center center;
    background-size: cover;
    
  }
  #NovBreadcrumbs .breadcrumb .list-inline-item a span, #NovBreadcrumbs .breadcrumb .list-inline-item span, #NovBreadcrumbs .headingPage {
    color: #ffffff;
  }
  #NovBreadcrumbs .breadcrumb .list-inline-item a:hover span {
    color: #ffffff;
  }






  
    
    
    
      
    
      
        
          
          Filter by
        
        
          Clear All



            
              
                Availability
                0 selected
              
              
                
                  Reset
                  0 selected
                

                
                      
                        
                        
                        In stock
                        42
                      
                    
                      
                        
                        
                        Out of stock
                        19
                      
                    
              
            
          

            
            
            

            
              
                
                  Price
                  
                
              
              
                The highest price is $39.95
                
                
                  
                    
                      From 
                    
                    
                      $
                      
                      
                    
                  
                  
                    
                      To
                    
                    
                      $
                      
                      
                    
                  
                
                
                  Reset
                
              
            
          

          
            Tags
          
          
            
              8 gtx coils
            
              a coil
            
              Accessories
            
              aegis boost mesh coil pack
            
              aegis boost mesh coil pack (5pk)
            
              aegis boost plus coils
            
              aegis boost pro p series coil
            
              aegis p series coil
            
              aegis p series coils
            
              ajax atomizer
            
              ajax coils
            
              ajax coils free shipping
            
              ajax plex 3d coil
            
              alien coils
            
              aspire
            
              aspire nautilus 2s coils 1 8 ohm
            
              aspire nautilus 2s coils australia
            
              aspire nautilus coils
            
              aspire nautilus coils australia
            
              aspire nautilus x coils australia
            
              baby v8 mesh coil
            
              caliburn g coil
            
              caliburn g coils australia
            
              caliburn g replacement
            
              ccell ceramic coils
            
              ceramic coil rda
            
              ceramic euc 0 3 ohm coil
            
              coil
            
              coils
            
              Core coil
            
              cube
            
              endura t18 coils australia
            
              endura t18 coils near me
            
              endura t18 compatible coils
            
              EUC
            
              euc ccell coil
            
              extra super mesh x2 coil 0 3 ohm
            
              Featured
            
              forz gtr coil 3pcs
            
              forz tx80
            
              freemax
            
              freemax m3
            
              game over man alien coils
            
              geek vape m replacement coils
            
              geekvape
            
              geekvape aegis boost cartridge
            
              geekvape aegis boost coils
            
              geekvape aegis boost plus cartridge
            
              geekvape aegis boost plus rba cartridge
            
              geekvape aegis boost pod cartridge
            
              geekvape aegis boost rba pod cartridge
            
              geekvape aegis empty pod cartridge 3.5ml
            
              geekvape aegis pod cartridge
            
              GEEKVAPE COILS
            
              geekvape coils australia
            
              geekvape m series coils 5pk
            
              geekvape m series coils 5pk australia
            
              geekvape m series coils 5pk price
            
              geekvape p series coil for aegis boost pro
            
              geekvape super mesh coil for aero shield cerberus
            
              geekvape zeus coils australia
            
              GRACEVILLE FOODMARKET
            
              gt ccell ceramic coils
            
              gt ccell coil
            
              gt mesh coil
            
              gti coils
            
              gtx coils
            
              gtx coils compatibility
            
              gtx coils near me
            
              gtx one coils
            
              IGA SPRINGFIELD LAKES
            
              IGA X-PRESS FOREST LAKE
            
              Innokin
            
              innokin ajax coil compatibility
            
              innokin ajax plex3d matrix coil 16ohm 5 pack
            
              innokin ajax tank
            
              INNOKIN COILS
            
              innokin endura t18 australia
            
              innokin endura t18 replacement coils 5 pack
            
              innokin mvp5 coils
            
              innokin prism t20 s replacement coil
            
              innokin replacement coils for ajax tank atomizer 5pcs
            
              innokin t18 replacement coils
            
              innokin t20s australia
            
              innokin t20s coils
            
              IPSWICH BREW CO
            
              JUNCTION NEWS ANNERLEY
            
              k pin mini coils
            
              kangertech 0.2 ohm coils
            
              kangertech 0.5 ohm coil
            
              kangertech australia
            
              kangertech clocc 1.5 coils
            
              kangertech clocc coils
            
              kangertech clocc coils 0.15
            
              kangertech clocc coils 0.5
            
              kangertech clocc coils 1 ohm
            
              kangertech coils australia
            
              kangertech ssocc coils australia
            
              kangertech subox mini australia
            
              KFB2
            
              KINKY KLOSETT
            
              m3
            
              mini accessories
            
              nautilus
            
              nikola antares coils
            
              nikola antares coils au
            
              nikola antares pod coils au
            
              nikola antares pod mod
            
              nikola antares replacement coils
            
              nrg gt core coil
            
              nunchaku coils
            
              obs
            
              orca solo
            
              orca solo coils
            
              orca solo replacement coils
            
              OXFORD STREET NEWS
            
              pnp coil
            
              prism s coil
            
              prism s replacement coil
            
              prism t20s
            
              qf strip coil
            
              smok
            
              smok 0.15 coil 40 80w best 60 70w
            
              smok 0.15 mesh coil
            
              smok 0.15 ohm 40 80 watts baby v8
            
              smok 0.15 ohm 40 80w
            
              smok coil
            
              smok coil 0.4 40 80w
            
              smok coils
            
              smok coils australia
            
              smok m17 coils near me
            
              smok m17 compatible coils
            
              smok priv m17 coil replacement
            
              smok stick aio coils near me
            
              smok stick aio compatible coils
            
              smok stick m17 coil compatibility
            
              smok stick m17 spare parts
            
              smok stick prince v12 coil
            
              smok stick replacement coils
            
              smok v12 prince coil
            
              smok v8 baby mesh coil amazon
            
              smok v8 t12 coil
            
              THRIFTY FOOD MARKET BETHANIA
            
              uwell
            
              uwell caliburn g coils
            
              uwell caliburn g replacement coil
            
              uwell crown
            
              uwell valyrian ii
            
              v12 prince coil compatibility
            
              v8 baby q2 0.4 dual coils
            
              v8 baby q2 coils
            
              v8 baby q4 coil
            
              v8 baby strip coil
            
              v8 baby strip replacement coils 0.15 ohm 5 pack
            
              vape king coils
            
              vape pen coils
            
              vape station
            
              vapefly optima coils
            
              vaporesso
            
              vaporesso ccell ceramic coils
            
              vaporesso ccell-gd ceramic replacement coils
            
              vaporesso ceramic coils
            
              vaporesso coils
            
              vaporesso euc coil review
            
              vaporesso euc coils 5 pack
            
              vaporesso euc coils australia
            
              vaporesso gt ccell 0 5 ohm coils
            
              vaporesso gt ccell ceramic coils
            
              vaporesso gt ccell coils review
            
              vaporesso gt coils australia
            
              vaporesso gt compatible coils
            
              vaporesso gtx coils compatibility
            
              vaporesso gtx one replacement coils
            
              vaporesso qf coils - skrr tank
            
              vaporesso qf coils australia
            
              vaporesso qf coils compatibility
            
              vaporesso skrr coils
            
              vaporesso skrr qf coils
            
              vaporesso skrr qf replacement coils
            
              vaporesso skrr qf vape coils
            
              vaporesso target mini ccell-gd ceramic coils for target tank
            
              vapour coils
            
              voopoo
            
              voopoo pnp coils
            
              voopoo pnp coils australia
            
              voopoo tpp
            
              voopoo tpp coil
            
              vsmajor
            
              vsmini
            
              x
            
              z coil
            
              zeus coil
            
          
        
      
      
    
  
  
    
      
        
          
          Filter by
          
        
        
  
  
  

      
      
        
          
            
              
                Filter by
                Showing 47 of 47 products
              
            
            
                  
                    
                      
                        Availability
                        
                        
                      
                    
                    
                      
                        Availability 
                      
                      
                            
                              
                              
                              In stock
                              42
                            
                          
                            
                              
                              
                              Out of stock
                              19
                            
                          

                      
                        Clear
                        Apply
                        &lt;button class=&quot;btn button button--primary&quot;>Apply&lt;/button>
                      
                    
                  
                

                  
                    
                      
                        Price
                        
                        
                      
                    
                    
                      
                        Price
                      

                      The highest price is $39.95

                      
                        
                          
                            From 
                          
                          
                            $
                            
                            
                          
                        
                        
                          
                            To 
                          
                          
                            $
                            
                            
                          
                        
                      
                    
                  
                

                  
                    
                      
                        FeaturedBest sellingAlphabetically, A-ZAlphabetically, Z-APrice, low to highPrice, high to lowDate, old to newDate, new to old
                      
                    
                  
                
                Clear
                Apply
                &lt;button class=&quot;btn button button--primary&quot;>Apply&lt;/button>
              
            

            
          
        
      
    
  

    
    
    
      
        
          
          
            
              
                
                
                
                
            
            
              
                
                  FeaturedBest sellingAlphabetically, A-ZAlphabetically, Z-APrice, low to highPrice, high to lowDate, old to newDate, new to old
                  
                
              
            
          
        
      
      
      
              
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/geekvape-zeus-replacement-coils-vape-station-586440_1024x.jpg?v=1663122428&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape Zeus coil (Z COIL)
                          
                          
                            

  
    $22.90
  


                          
                          Buy Geekvape Zeus Coil at Vape Station Redbank Plains.
The Geekvape Zeus Coil is best compatible with Geekvape Zeus Tank/Geekvape Aegis Legend Zeus Kit. 5 pieces each pack. Made of KA1, the Z Coil is a meshed coil for a longer ...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/vaporesso-gt-core-coils-3pk-default-vape-station-229145_1024x.jpg?v=1614226787&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Vaporesso GT Core Coil 3pk
                          
                          
                            

  
    $15.90
  


                          
                          Buy Vaporesso GT Core Coil at Vape Station Redbank Plains. 
Vaporesso GT Core Coil in 3packs is designed for NRG/ NRG SE tanks. We provide GT4, GT4 Meshed, GT8, GT CCELL, GT CCELL2, and GT Meshed coils for your selection. 
Para...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/euc-coils-ceramic-mesh-and-traditional-5-pack-accessories-vape-station-brown-ceramic-06-ohm-804550_1024x.jpg?v=1614226806&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                











    
        
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
    



                              
                            
                          
                          
                            
                              
                            
                          
                          
                            EUC coils ceramic, mesh and traditional (5 pack)
                          
                          
                            

  
    $17.95
  


                          
                          Product Introduction
5pcs Vaporesso Traditional/mesh and traditional EUC spare coil. Suitable for Veco Solo Plus 
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/aegis-p-series-coil-vape-station-716743_1024x.jpg?v=1663121557&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape P Coil
                          
                          
                            

  
    $21.90
  


                          
                          Buy the Geekvape P Coil at Vape Station Redbank Plains. 
It is best compatible with GeekVape Aegis Boost Pro Pod/GeekVape Aegis Boost Pro Pod Kit/GeekVape Aegis Boost Pro Empty Pod. And the Geekvape P Series Coil comes with 0.2...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/aegis-boost-mesh-coil-pack-5pk-default-vape-station-669366_1024x.jpg?v=1663120544&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape B COIL 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy Geekvape B Coil 5pcs at Vape Station Redbank Plains. 
The Geekvape Aegis Boost Replacement Coil is specially designed for Geekvape Aegis Boost Kit, Geekvape Aegis Boost Plus kit, Geekvape Aegis Boost Plus Pod Cartridge, and...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/caliburn-g-replacement-coil-vape-station-08ohm-265348_1024x.jpg?v=1663127373&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Uwell Caliburn G coil
                          
                          
                            

  
    $22.95
  


                          
                          Buy the Uwell Caliburn G coils at the Vape Station Australia store at Redbank Plains. They are compatible with the Caliburn G Pod Starter Kit only.
Specs:

UN2-Meshed H 0.8ohm: 13 - 18W (smoother airflow) 
FeCrAl 1.0ohm: (tight...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/supermesh-x1-coil-pack-5pk-default-vape-station-ka1-02ohm-103744_1024x.jpg?v=1663121933&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            Geekvape Supermesh Coil 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy GeekVape SuperMesh Coil at Vape Station.
Here comes the GeekVape SuperMesh coil designed for Shield tank, Aero tank, Aero Mesh tank and Cerberus subohm tank. 0.4ohm IM1 coil, 0.15ohm IM4 coil, 0.2ohm Mesh Coil are included ...
                          
  

    
      
      
        
          
            
          
        
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/drag-x-pnp-repacement-coils-vape-station-695680_1024x.jpg?v=1663123068&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            VOOPOO PNP Coil 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy Voopoo PNP Coil at Vape Station Redbank Plains. The VOOPOO PNP Coil is a versatile coil which is specially designed for the VOOPOO DRAG Baby Trio starter kit but is also used for many other Voopoo kits like the VOOPOO VINCI...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/orca-solo-replacement-coils-accessories-vape-station-334157_1024x.jpg?v=1601425725&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            Orca solo replacement coils 5pk
                          
                          
                            

  
    $19.95
  


                          
                          Orca solo replacement coils for orca solo 5pk
 
&quot;Keyword&quot;&quot;orca solo replacement coils 5pk&quot;&quot;orca solo coils&quot;&quot;orca solo&quot;
                          
  

    
      
      
        
          
            
          
        
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/gtx-coils-default-vape-station-gtx-mesh-03ohm-327879_1024x.jpg?v=1663118702&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Vaporesso GTX COIL 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Let's welcome the Vaporesso GTX Coil.  Vaporesso TARGET PM80 GTX Coil. is specially designed for Vaporesso Target PM80 Kit, Vaporesso Target PM80 Empty Pod, Vaporesso GTX One Kit, and Vaporesso GTX Tank 18. And the TARGET PM80 ...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/voopoo-tpp-mesh-coil-3pcs-vape-station-379866_1024x.jpg?v=1663123825&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            VOOPOO TPP Coil 3pcs
                          
                          
                            

  
    $17.95
  


                          
                          Buy Voopoo TPP Coil at Vape Station Redbank Plains store.
Take the VOOPOO TPP Coil for your VOOPOO TPP Pod Tank, VOOPOO DRAG X Plus Kit, VOOPOO DRAG 3 Kit. The TPP series coils feature a new patented technology that not only in...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/innokin-t18-replacement-coil-pack-5-default-vape-station-354944_1024x.jpg?v=1616564677&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                COIL PACKS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            INNOKIN - T18 Replacement Coil pack (5)
                          
                          
                            

  
    $17.95
  


                          
                          T18 replacement coil pack (5)
1.5 ohM horizontal coils. 
&quot;Keyword&quot;&quot;endura t18 coils australia&quot;&quot;endura t18 compatible coils&quot;&quot;innokin t18 replacement coils&quot;&quot;endura t18 coils near me&quot;&quot;innokin t18 replacement coils&quot;&quot;innokin endura ...
                          
  

    
      
        
      
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
  
  
  Showing 1 - 0 of 47 item(s)



  
    
      
        
        Previous
      
    
  

  
    
      
        1
      
    
  
    
      
        2
      
    
  
    
      
        3
      
    
  
    
      
        4
      
    
  

  
    
      
        Next
        
      
    
  



    
  



          
        

        

  
  
    
      
        
          
          
            
              
                
                  Contact Us
                  
                
              
              
                
                  
                    
                    Showroom: Shop 7, 319-321 Redbank Plains Road, Redbank Plains, QLD Australia
                  
                
                
                  
                    
                    07 3418 4513 OR 1300 355 535
                  
                
                
                  
                    
                    info@vapestation.co
                  
                
                
                  
                    
                    Opening Hours: MON - FRI - 10:00 am - 6:00 pm SAT - 10:00 am - 4:00pm  SUN - CLOSED
                  
                
              
            
          
        
          

              
                
                  USEFUL LINKS
                  
                
              
              
                
                  
                    
                      
                        GENERAL INFORMATION
                      
                    
                      
                        ZIP PAY
                      
                    
                      
                        PRIVACY POLICY
                      
                    
                      
                        REFUND POLICY
                      
                    
                      
                        TERMS OF SERVICE
                      
                    
                      
                        VAPE SAFETY
                      
                    
                  
                
              
            
          
          
        
          

              
                
                  Store Locator
                  
                
              
              
                
                  
                    
                      
                        
                      
                    
                  
                
              
            
          
          
         
      
    
  

  
    
      
        
          Copyright © 2022, VAPE STATION | Powered By Shopify | Developed by Bibidh Subedi and Eclogy.com
        
        
          
            
              
            
          
        
      
    
  



      
    </value>
      <webElementGuid>1b9354b3-68e1-4808-a0fb-d220b776f313</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js svg flexbox csstransforms&quot;]/body[@class=&quot;template-collection&quot;]/div[@class=&quot;overflow-hidden&quot;]</value>
      <webElementGuid>493d4714-0df5-4e88-87bd-71023eab530c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Yes'])[1]/following::div[1]</value>
      <webElementGuid>49608906-c2a8-4426-9c96-d1f8558653cc</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='No'])[1]/following::div[1]</value>
      <webElementGuid>00da5353-ce83-463e-a2ee-849868a2acbc</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]</value>
      <webElementGuid>683eaa75-e4e3-40c2-a042-b4f688c58501</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
      
        
          
        
      
      
        
          
        
      
      
        
          
            
            
            
          
        
        
          
            
              
              
            
          
        

        
  
    
      
        
          
        
        
          
            
              
              
            
          
        
        
          
        
       
      
        
  
  
  
    
      
      
    
  

      
    
    
       
          WE DO NOT SELL NICOTINE AS IT IS ILLEGAL TO SELL NICOTINE IN AUSTRALIA
      
    
    
      
        
          
            
              
                
                  
                    
                      
                      
                    
                  
                
              
              
            
          
          
            
              
                
                  
                    
  
  
  
    
      
      
    
  

                    
  $(function() {
    // Current Ajax request.
    var currentAjaxRequest = null;
    // Grabbing all search forms on the page, and adding a .search-results list to each.
    var searchForms = $(&quot; , &quot;'&quot; , &quot;form[action=&quot;/search&quot;]&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;position&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;relative&quot; , &quot;'&quot; , &quot;).each(function() {
      // Grabbing text input.
      var input = $(this).find(&quot; , &quot;'&quot; , &quot;input[name=&quot;q&quot;]&quot; , &quot;'&quot; , &quot;);
      // Adding a list for showing search results.
      if ($(&quot; , &quot;'&quot; , &quot;.template-search&quot; , &quot;'&quot; , &quot;).length > 0) {
        var offSet = input.position().top + input.innerHeight() + 50;
      } else {
        if ($(window).width() > 1199) {
        var offSet = input.position().top + input.innerHeight() + 177;
        } else {
          var offSet = input.position().top + input.innerHeight() + 107;
        }
      }
      $(&quot; , &quot;'&quot; , &quot;&lt;ul class=&quot;search-results has-scroll row&quot;>&lt;/ul>&quot; , &quot;'&quot; , &quot;).appendTo($(this)).wrap(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;block_search_autocomplete&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;.block_search_autocomplete&quot; , &quot;'&quot; , &quot;).css( { &quot; , &quot;'&quot; , &quot;position&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;0px&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;71px&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;z-index&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;99&quot; , &quot;'&quot; , &quot; } ).hide();
      $(&quot; , &quot;'&quot; , &quot;.search_trend&quot; , &quot;'&quot; , &quot;).hide();
      // Listening to keyup and change on the text field within these search forms.
      input.attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;off&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;keyup change&quot; , &quot;'&quot; , &quot;, function() {
        // What&quot; , &quot;'&quot; , &quot;s the search term?
        var term = $(this).val();
        // What&quot; , &quot;'&quot; , &quot;s the search form?
        var form = $(this).closest(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;);
        // What&quot; , &quot;'&quot; , &quot;s the search URL?
        var searchURL = &quot; , &quot;'&quot; , &quot;/search?type=product&amp;q=&quot; , &quot;'&quot; , &quot; + term;
        // What&quot; , &quot;'&quot; , &quot;s the search results list?
        var resultsList = form.find(&quot; , &quot;'&quot; , &quot;.search-results&quot; , &quot;'&quot; , &quot;);
        // If that&quot; , &quot;'&quot; , &quot;s a new term and it contains at least 3 characters.
        if (term.length > 3 &amp;&amp; term != $(this).attr(&quot; , &quot;'&quot; , &quot;data-old-term&quot; , &quot;'&quot; , &quot;)) {
          // Saving old query.
          $(this).attr(&quot; , &quot;'&quot; , &quot;data-old-term&quot; , &quot;'&quot; , &quot;, term);
          // Killing any Ajax request that&quot; , &quot;'&quot; , &quot;s currently being processed.
          if (currentAjaxRequest != null) currentAjaxRequest.abort();
          // Pulling results.
          currentAjaxRequest = $.getJSON(searchURL + &quot; , &quot;'&quot; , &quot;&amp;view=json&quot; , &quot;'&quot; , &quot;, function(data) {
            // Reset results.
            resultsList.empty();
            // If we have no results.
            if(data.results_count == 0) {
              // resultsList.html(&quot; , &quot;'&quot; , &quot;&lt;li>&lt;span class=&quot;title&quot;>No results.&lt;/span>&lt;/li>&quot; , &quot;'&quot; , &quot;);
              // resultsList.fadeIn(200);
              $(&quot; , &quot;'&quot; , &quot;.block_search_autocomplete&quot; , &quot;'&quot; , &quot;).hide();
              $(&quot; , &quot;'&quot; , &quot;.search_trend&quot; , &quot;'&quot; , &quot;).hide();
            } else {
              // If we have results.
              $.each(data.results, function(index, item) {
                var link = $(&quot; , &quot;'&quot; , &quot;&lt;a>&lt;/a>&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, item.url);
                link.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;thumbnail&quot;>&lt;img src=&quot;&quot; , &quot;'&quot; , &quot; + item.thumbnail + &quot; , &quot;'&quot; , &quot;&quot; />&lt;/div>&quot; , &quot;'&quot; , &quot;);
                link.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;media-body&quot;>&lt;div class=&quot;title&quot;>&quot; , &quot;'&quot; , &quot; + item.title + &quot; , &quot;'&quot; , &quot;&lt;/div>&lt;div class=&quot;price&quot;>&quot; , &quot;'&quot; , &quot; + item.price + &quot; , &quot;'&quot; , &quot;&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
                //link.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;price&quot;>&quot; , &quot;'&quot; , &quot; + item.price + &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;);
                link.wrap(&quot; , &quot;'&quot; , &quot;&lt;li class=&quot;col-lg-cus-5 col-md-cus-5&quot;>&lt;/li>&quot; , &quot;'&quot; , &quot;);
                resultsList.append(link.parent());
              });
              // The Ajax request will return at the most 10 results.
              // If there are more than 10, let&quot; , &quot;'&quot; , &quot;s link to the search results page.
              if(data.results_count > 4) {
                resultsList.append(&quot; , &quot;'&quot; , &quot;&lt;li>&lt;a class=&quot;see_all&quot; href=&quot;&quot; , &quot;'&quot; , &quot; + searchURL + &quot; , &quot;'&quot; , &quot;&quot;>See all results (&quot; , &quot;'&quot; , &quot; + data.results_count + &quot; , &quot;'&quot; , &quot;)&lt;/a>&lt;/li>&quot; , &quot;'&quot; , &quot;);
              }
              $(&quot; , &quot;'&quot; , &quot;.block_search_autocomplete&quot; , &quot;'&quot; , &quot;).fadeIn(200);
              $(&quot; , &quot;'&quot; , &quot;.search_trend&quot; , &quot;'&quot; , &quot;).show();
            }        
          });
        }
      });
    });
    // Clicking outside makes the results disappear.
    $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(){
      $(&quot; , &quot;'&quot; , &quot;.block_search_autocomplete&quot; , &quot;'&quot; , &quot;).hide();
      $(&quot; , &quot;'&quot; , &quot;.search_trend&quot; , &quot;'&quot; , &quot;).hide();
    });
  });

                  
                
              
            
            
              
            
            
              
                
                
                  
                    
                      
                      
                        
                      
                    
                  
                
              
            
          
        
      
    
    
      
        
          
          
            
  
    
    
    


    
      
    

    

    

      
        
          
            60ml VS E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            100ml VS E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            INTERSTATE &amp; INTERNATIONAL E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            COILS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            KITS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            MODS 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            TANKS 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            BATTERIES &amp; CHARGERS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            REPLACEMENT GLASS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            DIY
          
          
            
              
                
                  
                    
                      DIY PRODUCTS
                    
                  
                    
                      DIY PREMIUM CONCENTRATES
                    
                  
                
              
            
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            VS STANDARD CONCENTRATES 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            STORE LOCATOR
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            BLOGS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            Nicotine Script
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            ADULT PRODUCTS
          
        
      

    
  


          
          
  
    

    

    

    
    
    

    

    

    

    

    

    
  

        
      
    
  
  



        
          
            


  
    
      
        COILS
      
      
        
          
            Home
          
          
        
        
          
            
              COILS
              
            
          
        
      
      
      
    
  


  #NovBreadcrumbs::before {
    content: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    display: block;
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    background: rgba(0, 0, 0, 50%);
  }
  
  #NovBreadcrumbs {
    padding-top: 145px;
    padding-bottom: 126px;
    
    
    background-image: url(&quot;//cdn.shopify.com/s/files/1/2663/3040/files/breadcrumb-background_1920x.jpg?v=1651648211&quot;);
    background-repeat: no-repeat;
    background-position: center center;
    background-size: cover;
    
  }
  #NovBreadcrumbs .breadcrumb .list-inline-item a span, #NovBreadcrumbs .breadcrumb .list-inline-item span, #NovBreadcrumbs .headingPage {
    color: #ffffff;
  }
  #NovBreadcrumbs .breadcrumb .list-inline-item a:hover span {
    color: #ffffff;
  }






  
    
    
    
      
    
      
        
          
          Filter by
        
        
          Clear All



            
              
                Availability
                0 selected
              
              
                
                  Reset
                  0 selected
                

                
                      
                        
                        
                        In stock
                        42
                      
                    
                      
                        
                        
                        Out of stock
                        19
                      
                    
              
            
          

            
            
            

            
              
                
                  Price
                  
                
              
              
                The highest price is $39.95
                
                
                  
                    
                      From 
                    
                    
                      $
                      
                      
                    
                  
                  
                    
                      To
                    
                    
                      $
                      
                      
                    
                  
                
                
                  Reset
                
              
            
          

          
            Tags
          
          
            
              8 gtx coils
            
              a coil
            
              Accessories
            
              aegis boost mesh coil pack
            
              aegis boost mesh coil pack (5pk)
            
              aegis boost plus coils
            
              aegis boost pro p series coil
            
              aegis p series coil
            
              aegis p series coils
            
              ajax atomizer
            
              ajax coils
            
              ajax coils free shipping
            
              ajax plex 3d coil
            
              alien coils
            
              aspire
            
              aspire nautilus 2s coils 1 8 ohm
            
              aspire nautilus 2s coils australia
            
              aspire nautilus coils
            
              aspire nautilus coils australia
            
              aspire nautilus x coils australia
            
              baby v8 mesh coil
            
              caliburn g coil
            
              caliburn g coils australia
            
              caliburn g replacement
            
              ccell ceramic coils
            
              ceramic coil rda
            
              ceramic euc 0 3 ohm coil
            
              coil
            
              coils
            
              Core coil
            
              cube
            
              endura t18 coils australia
            
              endura t18 coils near me
            
              endura t18 compatible coils
            
              EUC
            
              euc ccell coil
            
              extra super mesh x2 coil 0 3 ohm
            
              Featured
            
              forz gtr coil 3pcs
            
              forz tx80
            
              freemax
            
              freemax m3
            
              game over man alien coils
            
              geek vape m replacement coils
            
              geekvape
            
              geekvape aegis boost cartridge
            
              geekvape aegis boost coils
            
              geekvape aegis boost plus cartridge
            
              geekvape aegis boost plus rba cartridge
            
              geekvape aegis boost pod cartridge
            
              geekvape aegis boost rba pod cartridge
            
              geekvape aegis empty pod cartridge 3.5ml
            
              geekvape aegis pod cartridge
            
              GEEKVAPE COILS
            
              geekvape coils australia
            
              geekvape m series coils 5pk
            
              geekvape m series coils 5pk australia
            
              geekvape m series coils 5pk price
            
              geekvape p series coil for aegis boost pro
            
              geekvape super mesh coil for aero shield cerberus
            
              geekvape zeus coils australia
            
              GRACEVILLE FOODMARKET
            
              gt ccell ceramic coils
            
              gt ccell coil
            
              gt mesh coil
            
              gti coils
            
              gtx coils
            
              gtx coils compatibility
            
              gtx coils near me
            
              gtx one coils
            
              IGA SPRINGFIELD LAKES
            
              IGA X-PRESS FOREST LAKE
            
              Innokin
            
              innokin ajax coil compatibility
            
              innokin ajax plex3d matrix coil 16ohm 5 pack
            
              innokin ajax tank
            
              INNOKIN COILS
            
              innokin endura t18 australia
            
              innokin endura t18 replacement coils 5 pack
            
              innokin mvp5 coils
            
              innokin prism t20 s replacement coil
            
              innokin replacement coils for ajax tank atomizer 5pcs
            
              innokin t18 replacement coils
            
              innokin t20s australia
            
              innokin t20s coils
            
              IPSWICH BREW CO
            
              JUNCTION NEWS ANNERLEY
            
              k pin mini coils
            
              kangertech 0.2 ohm coils
            
              kangertech 0.5 ohm coil
            
              kangertech australia
            
              kangertech clocc 1.5 coils
            
              kangertech clocc coils
            
              kangertech clocc coils 0.15
            
              kangertech clocc coils 0.5
            
              kangertech clocc coils 1 ohm
            
              kangertech coils australia
            
              kangertech ssocc coils australia
            
              kangertech subox mini australia
            
              KFB2
            
              KINKY KLOSETT
            
              m3
            
              mini accessories
            
              nautilus
            
              nikola antares coils
            
              nikola antares coils au
            
              nikola antares pod coils au
            
              nikola antares pod mod
            
              nikola antares replacement coils
            
              nrg gt core coil
            
              nunchaku coils
            
              obs
            
              orca solo
            
              orca solo coils
            
              orca solo replacement coils
            
              OXFORD STREET NEWS
            
              pnp coil
            
              prism s coil
            
              prism s replacement coil
            
              prism t20s
            
              qf strip coil
            
              smok
            
              smok 0.15 coil 40 80w best 60 70w
            
              smok 0.15 mesh coil
            
              smok 0.15 ohm 40 80 watts baby v8
            
              smok 0.15 ohm 40 80w
            
              smok coil
            
              smok coil 0.4 40 80w
            
              smok coils
            
              smok coils australia
            
              smok m17 coils near me
            
              smok m17 compatible coils
            
              smok priv m17 coil replacement
            
              smok stick aio coils near me
            
              smok stick aio compatible coils
            
              smok stick m17 coil compatibility
            
              smok stick m17 spare parts
            
              smok stick prince v12 coil
            
              smok stick replacement coils
            
              smok v12 prince coil
            
              smok v8 baby mesh coil amazon
            
              smok v8 t12 coil
            
              THRIFTY FOOD MARKET BETHANIA
            
              uwell
            
              uwell caliburn g coils
            
              uwell caliburn g replacement coil
            
              uwell crown
            
              uwell valyrian ii
            
              v12 prince coil compatibility
            
              v8 baby q2 0.4 dual coils
            
              v8 baby q2 coils
            
              v8 baby q4 coil
            
              v8 baby strip coil
            
              v8 baby strip replacement coils 0.15 ohm 5 pack
            
              vape king coils
            
              vape pen coils
            
              vape station
            
              vapefly optima coils
            
              vaporesso
            
              vaporesso ccell ceramic coils
            
              vaporesso ccell-gd ceramic replacement coils
            
              vaporesso ceramic coils
            
              vaporesso coils
            
              vaporesso euc coil review
            
              vaporesso euc coils 5 pack
            
              vaporesso euc coils australia
            
              vaporesso gt ccell 0 5 ohm coils
            
              vaporesso gt ccell ceramic coils
            
              vaporesso gt ccell coils review
            
              vaporesso gt coils australia
            
              vaporesso gt compatible coils
            
              vaporesso gtx coils compatibility
            
              vaporesso gtx one replacement coils
            
              vaporesso qf coils - skrr tank
            
              vaporesso qf coils australia
            
              vaporesso qf coils compatibility
            
              vaporesso skrr coils
            
              vaporesso skrr qf coils
            
              vaporesso skrr qf replacement coils
            
              vaporesso skrr qf vape coils
            
              vaporesso target mini ccell-gd ceramic coils for target tank
            
              vapour coils
            
              voopoo
            
              voopoo pnp coils
            
              voopoo pnp coils australia
            
              voopoo tpp
            
              voopoo tpp coil
            
              vsmajor
            
              vsmini
            
              x
            
              z coil
            
              zeus coil
            
          
        
      
      
    
  
  
    
      
        
          
          Filter by
          
        
        
  
  
  

      
      
        
          
            
              
                Filter by
                Showing 47 of 47 products
              
            
            
                  
                    
                      
                        Availability
                        
                        
                      
                    
                    
                      
                        Availability 
                      
                      
                            
                              
                              
                              In stock
                              42
                            
                          
                            
                              
                              
                              Out of stock
                              19
                            
                          

                      
                        Clear
                        Apply
                        &lt;button class=&quot;btn button button--primary&quot;>Apply&lt;/button>
                      
                    
                  
                

                  
                    
                      
                        Price
                        
                        
                      
                    
                    
                      
                        Price
                      

                      The highest price is $39.95

                      
                        
                          
                            From 
                          
                          
                            $
                            
                            
                          
                        
                        
                          
                            To 
                          
                          
                            $
                            
                            
                          
                        
                      
                    
                  
                

                  
                    
                      
                        FeaturedBest sellingAlphabetically, A-ZAlphabetically, Z-APrice, low to highPrice, high to lowDate, old to newDate, new to old
                      
                    
                  
                
                Clear
                Apply
                &lt;button class=&quot;btn button button--primary&quot;>Apply&lt;/button>
              
            

            
          
        
      
    
  

    
    
    
      
        
          
          
            
              
                
                
                
                
            
            
              
                
                  FeaturedBest sellingAlphabetically, A-ZAlphabetically, Z-APrice, low to highPrice, high to lowDate, old to newDate, new to old
                  
                
              
            
          
        
      
      
      
              
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/geekvape-zeus-replacement-coils-vape-station-586440_1024x.jpg?v=1663122428&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape Zeus coil (Z COIL)
                          
                          
                            

  
    $22.90
  


                          
                          Buy Geekvape Zeus Coil at Vape Station Redbank Plains.
The Geekvape Zeus Coil is best compatible with Geekvape Zeus Tank/Geekvape Aegis Legend Zeus Kit. 5 pieces each pack. Made of KA1, the Z Coil is a meshed coil for a longer ...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/vaporesso-gt-core-coils-3pk-default-vape-station-229145_1024x.jpg?v=1614226787&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Vaporesso GT Core Coil 3pk
                          
                          
                            

  
    $15.90
  


                          
                          Buy Vaporesso GT Core Coil at Vape Station Redbank Plains. 
Vaporesso GT Core Coil in 3packs is designed for NRG/ NRG SE tanks. We provide GT4, GT4 Meshed, GT8, GT CCELL, GT CCELL2, and GT Meshed coils for your selection. 
Para...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/euc-coils-ceramic-mesh-and-traditional-5-pack-accessories-vape-station-brown-ceramic-06-ohm-804550_1024x.jpg?v=1614226806&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                











    
        
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
    



                              
                            
                          
                          
                            
                              
                            
                          
                          
                            EUC coils ceramic, mesh and traditional (5 pack)
                          
                          
                            

  
    $17.95
  


                          
                          Product Introduction
5pcs Vaporesso Traditional/mesh and traditional EUC spare coil. Suitable for Veco Solo Plus 
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/aegis-p-series-coil-vape-station-716743_1024x.jpg?v=1663121557&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape P Coil
                          
                          
                            

  
    $21.90
  


                          
                          Buy the Geekvape P Coil at Vape Station Redbank Plains. 
It is best compatible with GeekVape Aegis Boost Pro Pod/GeekVape Aegis Boost Pro Pod Kit/GeekVape Aegis Boost Pro Empty Pod. And the Geekvape P Series Coil comes with 0.2...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/aegis-boost-mesh-coil-pack-5pk-default-vape-station-669366_1024x.jpg?v=1663120544&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape B COIL 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy Geekvape B Coil 5pcs at Vape Station Redbank Plains. 
The Geekvape Aegis Boost Replacement Coil is specially designed for Geekvape Aegis Boost Kit, Geekvape Aegis Boost Plus kit, Geekvape Aegis Boost Plus Pod Cartridge, and...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/caliburn-g-replacement-coil-vape-station-08ohm-265348_1024x.jpg?v=1663127373&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Uwell Caliburn G coil
                          
                          
                            

  
    $22.95
  


                          
                          Buy the Uwell Caliburn G coils at the Vape Station Australia store at Redbank Plains. They are compatible with the Caliburn G Pod Starter Kit only.
Specs:

UN2-Meshed H 0.8ohm: 13 - 18W (smoother airflow) 
FeCrAl 1.0ohm: (tight...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/supermesh-x1-coil-pack-5pk-default-vape-station-ka1-02ohm-103744_1024x.jpg?v=1663121933&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            Geekvape Supermesh Coil 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy GeekVape SuperMesh Coil at Vape Station.
Here comes the GeekVape SuperMesh coil designed for Shield tank, Aero tank, Aero Mesh tank and Cerberus subohm tank. 0.4ohm IM1 coil, 0.15ohm IM4 coil, 0.2ohm Mesh Coil are included ...
                          
  

    
      
      
        
          
            
          
        
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/drag-x-pnp-repacement-coils-vape-station-695680_1024x.jpg?v=1663123068&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            VOOPOO PNP Coil 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy Voopoo PNP Coil at Vape Station Redbank Plains. The VOOPOO PNP Coil is a versatile coil which is specially designed for the VOOPOO DRAG Baby Trio starter kit but is also used for many other Voopoo kits like the VOOPOO VINCI...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/orca-solo-replacement-coils-accessories-vape-station-334157_1024x.jpg?v=1601425725&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            Orca solo replacement coils 5pk
                          
                          
                            

  
    $19.95
  


                          
                          Orca solo replacement coils for orca solo 5pk
 
&quot;Keyword&quot;&quot;orca solo replacement coils 5pk&quot;&quot;orca solo coils&quot;&quot;orca solo&quot;
                          
  

    
      
      
        
          
            
          
        
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/gtx-coils-default-vape-station-gtx-mesh-03ohm-327879_1024x.jpg?v=1663118702&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Vaporesso GTX COIL 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Let&quot; , &quot;'&quot; , &quot;s welcome the Vaporesso GTX Coil.  Vaporesso TARGET PM80 GTX Coil. is specially designed for Vaporesso Target PM80 Kit, Vaporesso Target PM80 Empty Pod, Vaporesso GTX One Kit, and Vaporesso GTX Tank 18. And the TARGET PM80 ...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/voopoo-tpp-mesh-coil-3pcs-vape-station-379866_1024x.jpg?v=1663123825&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            VOOPOO TPP Coil 3pcs
                          
                          
                            

  
    $17.95
  


                          
                          Buy Voopoo TPP Coil at Vape Station Redbank Plains store.
Take the VOOPOO TPP Coil for your VOOPOO TPP Pod Tank, VOOPOO DRAG X Plus Kit, VOOPOO DRAG 3 Kit. The TPP series coils feature a new patented technology that not only in...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/innokin-t18-replacement-coil-pack-5-default-vape-station-354944_1024x.jpg?v=1616564677&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                COIL PACKS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            INNOKIN - T18 Replacement Coil pack (5)
                          
                          
                            

  
    $17.95
  


                          
                          T18 replacement coil pack (5)
1.5 ohM horizontal coils. 
&quot;Keyword&quot;&quot;endura t18 coils australia&quot;&quot;endura t18 compatible coils&quot;&quot;innokin t18 replacement coils&quot;&quot;endura t18 coils near me&quot;&quot;innokin t18 replacement coils&quot;&quot;innokin endura ...
                          
  

    
      
        
      
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
  
  
  Showing 1 - 0 of 47 item(s)



  
    
      
        
        Previous
      
    
  

  
    
      
        1
      
    
  
    
      
        2
      
    
  
    
      
        3
      
    
  
    
      
        4
      
    
  

  
    
      
        Next
        
      
    
  



    
  



          
        

        

  
  
    
      
        
          
          
            
              
                
                  Contact Us
                  
                
              
              
                
                  
                    
                    Showroom: Shop 7, 319-321 Redbank Plains Road, Redbank Plains, QLD Australia
                  
                
                
                  
                    
                    07 3418 4513 OR 1300 355 535
                  
                
                
                  
                    
                    info@vapestation.co
                  
                
                
                  
                    
                    Opening Hours: MON - FRI - 10:00 am - 6:00 pm SAT - 10:00 am - 4:00pm  SUN - CLOSED
                  
                
              
            
          
        
          

              
                
                  USEFUL LINKS
                  
                
              
              
                
                  
                    
                      
                        GENERAL INFORMATION
                      
                    
                      
                        ZIP PAY
                      
                    
                      
                        PRIVACY POLICY
                      
                    
                      
                        REFUND POLICY
                      
                    
                      
                        TERMS OF SERVICE
                      
                    
                      
                        VAPE SAFETY
                      
                    
                  
                
              
            
          
          
        
          

              
                
                  Store Locator
                  
                
              
              
                
                  
                    
                      
                        
                      
                    
                  
                
              
            
          
          
         
      
    
  

  
    
      
        
          Copyright © 2022, VAPE STATION | Powered By Shopify | Developed by Bibidh Subedi and Eclogy.com
        
        
          
            
              
            
          
        
      
    
  



      
    &quot;) or . = concat(&quot;
      
        
          
        
      
      
        
          
        
      
      
        
          
            
            
            
          
        
        
          
            
              
              
            
          
        

        
  
    
      
        
          
        
        
          
            
              
              
            
          
        
        
          
        
       
      
        
  
  
  
    
      
      
    
  

      
    
    
       
          WE DO NOT SELL NICOTINE AS IT IS ILLEGAL TO SELL NICOTINE IN AUSTRALIA
      
    
    
      
        
          
            
              
                
                  
                    
                      
                      
                    
                  
                
              
              
            
          
          
            
              
                
                  
                    
  
  
  
    
      
      
    
  

                    
  $(function() {
    // Current Ajax request.
    var currentAjaxRequest = null;
    // Grabbing all search forms on the page, and adding a .search-results list to each.
    var searchForms = $(&quot; , &quot;'&quot; , &quot;form[action=&quot;/search&quot;]&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;position&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;relative&quot; , &quot;'&quot; , &quot;).each(function() {
      // Grabbing text input.
      var input = $(this).find(&quot; , &quot;'&quot; , &quot;input[name=&quot;q&quot;]&quot; , &quot;'&quot; , &quot;);
      // Adding a list for showing search results.
      if ($(&quot; , &quot;'&quot; , &quot;.template-search&quot; , &quot;'&quot; , &quot;).length > 0) {
        var offSet = input.position().top + input.innerHeight() + 50;
      } else {
        if ($(window).width() > 1199) {
        var offSet = input.position().top + input.innerHeight() + 177;
        } else {
          var offSet = input.position().top + input.innerHeight() + 107;
        }
      }
      $(&quot; , &quot;'&quot; , &quot;&lt;ul class=&quot;search-results has-scroll row&quot;>&lt;/ul>&quot; , &quot;'&quot; , &quot;).appendTo($(this)).wrap(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;block_search_autocomplete&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;.block_search_autocomplete&quot; , &quot;'&quot; , &quot;).css( { &quot; , &quot;'&quot; , &quot;position&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;0px&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;71px&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;z-index&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;99&quot; , &quot;'&quot; , &quot; } ).hide();
      $(&quot; , &quot;'&quot; , &quot;.search_trend&quot; , &quot;'&quot; , &quot;).hide();
      // Listening to keyup and change on the text field within these search forms.
      input.attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;off&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;keyup change&quot; , &quot;'&quot; , &quot;, function() {
        // What&quot; , &quot;'&quot; , &quot;s the search term?
        var term = $(this).val();
        // What&quot; , &quot;'&quot; , &quot;s the search form?
        var form = $(this).closest(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;);
        // What&quot; , &quot;'&quot; , &quot;s the search URL?
        var searchURL = &quot; , &quot;'&quot; , &quot;/search?type=product&amp;q=&quot; , &quot;'&quot; , &quot; + term;
        // What&quot; , &quot;'&quot; , &quot;s the search results list?
        var resultsList = form.find(&quot; , &quot;'&quot; , &quot;.search-results&quot; , &quot;'&quot; , &quot;);
        // If that&quot; , &quot;'&quot; , &quot;s a new term and it contains at least 3 characters.
        if (term.length > 3 &amp;&amp; term != $(this).attr(&quot; , &quot;'&quot; , &quot;data-old-term&quot; , &quot;'&quot; , &quot;)) {
          // Saving old query.
          $(this).attr(&quot; , &quot;'&quot; , &quot;data-old-term&quot; , &quot;'&quot; , &quot;, term);
          // Killing any Ajax request that&quot; , &quot;'&quot; , &quot;s currently being processed.
          if (currentAjaxRequest != null) currentAjaxRequest.abort();
          // Pulling results.
          currentAjaxRequest = $.getJSON(searchURL + &quot; , &quot;'&quot; , &quot;&amp;view=json&quot; , &quot;'&quot; , &quot;, function(data) {
            // Reset results.
            resultsList.empty();
            // If we have no results.
            if(data.results_count == 0) {
              // resultsList.html(&quot; , &quot;'&quot; , &quot;&lt;li>&lt;span class=&quot;title&quot;>No results.&lt;/span>&lt;/li>&quot; , &quot;'&quot; , &quot;);
              // resultsList.fadeIn(200);
              $(&quot; , &quot;'&quot; , &quot;.block_search_autocomplete&quot; , &quot;'&quot; , &quot;).hide();
              $(&quot; , &quot;'&quot; , &quot;.search_trend&quot; , &quot;'&quot; , &quot;).hide();
            } else {
              // If we have results.
              $.each(data.results, function(index, item) {
                var link = $(&quot; , &quot;'&quot; , &quot;&lt;a>&lt;/a>&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, item.url);
                link.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;thumbnail&quot;>&lt;img src=&quot;&quot; , &quot;'&quot; , &quot; + item.thumbnail + &quot; , &quot;'&quot; , &quot;&quot; />&lt;/div>&quot; , &quot;'&quot; , &quot;);
                link.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;media-body&quot;>&lt;div class=&quot;title&quot;>&quot; , &quot;'&quot; , &quot; + item.title + &quot; , &quot;'&quot; , &quot;&lt;/div>&lt;div class=&quot;price&quot;>&quot; , &quot;'&quot; , &quot; + item.price + &quot; , &quot;'&quot; , &quot;&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
                //link.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;price&quot;>&quot; , &quot;'&quot; , &quot; + item.price + &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;);
                link.wrap(&quot; , &quot;'&quot; , &quot;&lt;li class=&quot;col-lg-cus-5 col-md-cus-5&quot;>&lt;/li>&quot; , &quot;'&quot; , &quot;);
                resultsList.append(link.parent());
              });
              // The Ajax request will return at the most 10 results.
              // If there are more than 10, let&quot; , &quot;'&quot; , &quot;s link to the search results page.
              if(data.results_count > 4) {
                resultsList.append(&quot; , &quot;'&quot; , &quot;&lt;li>&lt;a class=&quot;see_all&quot; href=&quot;&quot; , &quot;'&quot; , &quot; + searchURL + &quot; , &quot;'&quot; , &quot;&quot;>See all results (&quot; , &quot;'&quot; , &quot; + data.results_count + &quot; , &quot;'&quot; , &quot;)&lt;/a>&lt;/li>&quot; , &quot;'&quot; , &quot;);
              }
              $(&quot; , &quot;'&quot; , &quot;.block_search_autocomplete&quot; , &quot;'&quot; , &quot;).fadeIn(200);
              $(&quot; , &quot;'&quot; , &quot;.search_trend&quot; , &quot;'&quot; , &quot;).show();
            }        
          });
        }
      });
    });
    // Clicking outside makes the results disappear.
    $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(){
      $(&quot; , &quot;'&quot; , &quot;.block_search_autocomplete&quot; , &quot;'&quot; , &quot;).hide();
      $(&quot; , &quot;'&quot; , &quot;.search_trend&quot; , &quot;'&quot; , &quot;).hide();
    });
  });

                  
                
              
            
            
              
            
            
              
                
                
                  
                    
                      
                      
                        
                      
                    
                  
                
              
            
          
        
      
    
    
      
        
          
          
            
  
    
    
    


    
      
    

    

    

      
        
          
            60ml VS E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            100ml VS E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            INTERSTATE &amp; INTERNATIONAL E-LIQUIDS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            COILS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            KITS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            MODS 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            TANKS 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            BATTERIES &amp; CHARGERS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            REPLACEMENT GLASS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            DIY
          
          
            
              
                
                  
                    
                      DIY PRODUCTS
                    
                  
                    
                      DIY PREMIUM CONCENTRATES
                    
                  
                
              
            
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            VS STANDARD CONCENTRATES 
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            STORE LOCATOR
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            BLOGS
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            Nicotine Script
          
        
      

    
  
    
    
    


    
      
    

    

    

      
        
          
            ADULT PRODUCTS
          
        
      

    
  


          
          
  
    

    

    

    
    
    

    

    

    

    

    

    
  

        
      
    
  
  



        
          
            


  
    
      
        COILS
      
      
        
          
            Home
          
          
        
        
          
            
              COILS
              
            
          
        
      
      
      
    
  


  #NovBreadcrumbs::before {
    content: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    display: block;
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    background: rgba(0, 0, 0, 50%);
  }
  
  #NovBreadcrumbs {
    padding-top: 145px;
    padding-bottom: 126px;
    
    
    background-image: url(&quot;//cdn.shopify.com/s/files/1/2663/3040/files/breadcrumb-background_1920x.jpg?v=1651648211&quot;);
    background-repeat: no-repeat;
    background-position: center center;
    background-size: cover;
    
  }
  #NovBreadcrumbs .breadcrumb .list-inline-item a span, #NovBreadcrumbs .breadcrumb .list-inline-item span, #NovBreadcrumbs .headingPage {
    color: #ffffff;
  }
  #NovBreadcrumbs .breadcrumb .list-inline-item a:hover span {
    color: #ffffff;
  }






  
    
    
    
      
    
      
        
          
          Filter by
        
        
          Clear All



            
              
                Availability
                0 selected
              
              
                
                  Reset
                  0 selected
                

                
                      
                        
                        
                        In stock
                        42
                      
                    
                      
                        
                        
                        Out of stock
                        19
                      
                    
              
            
          

            
            
            

            
              
                
                  Price
                  
                
              
              
                The highest price is $39.95
                
                
                  
                    
                      From 
                    
                    
                      $
                      
                      
                    
                  
                  
                    
                      To
                    
                    
                      $
                      
                      
                    
                  
                
                
                  Reset
                
              
            
          

          
            Tags
          
          
            
              8 gtx coils
            
              a coil
            
              Accessories
            
              aegis boost mesh coil pack
            
              aegis boost mesh coil pack (5pk)
            
              aegis boost plus coils
            
              aegis boost pro p series coil
            
              aegis p series coil
            
              aegis p series coils
            
              ajax atomizer
            
              ajax coils
            
              ajax coils free shipping
            
              ajax plex 3d coil
            
              alien coils
            
              aspire
            
              aspire nautilus 2s coils 1 8 ohm
            
              aspire nautilus 2s coils australia
            
              aspire nautilus coils
            
              aspire nautilus coils australia
            
              aspire nautilus x coils australia
            
              baby v8 mesh coil
            
              caliburn g coil
            
              caliburn g coils australia
            
              caliburn g replacement
            
              ccell ceramic coils
            
              ceramic coil rda
            
              ceramic euc 0 3 ohm coil
            
              coil
            
              coils
            
              Core coil
            
              cube
            
              endura t18 coils australia
            
              endura t18 coils near me
            
              endura t18 compatible coils
            
              EUC
            
              euc ccell coil
            
              extra super mesh x2 coil 0 3 ohm
            
              Featured
            
              forz gtr coil 3pcs
            
              forz tx80
            
              freemax
            
              freemax m3
            
              game over man alien coils
            
              geek vape m replacement coils
            
              geekvape
            
              geekvape aegis boost cartridge
            
              geekvape aegis boost coils
            
              geekvape aegis boost plus cartridge
            
              geekvape aegis boost plus rba cartridge
            
              geekvape aegis boost pod cartridge
            
              geekvape aegis boost rba pod cartridge
            
              geekvape aegis empty pod cartridge 3.5ml
            
              geekvape aegis pod cartridge
            
              GEEKVAPE COILS
            
              geekvape coils australia
            
              geekvape m series coils 5pk
            
              geekvape m series coils 5pk australia
            
              geekvape m series coils 5pk price
            
              geekvape p series coil for aegis boost pro
            
              geekvape super mesh coil for aero shield cerberus
            
              geekvape zeus coils australia
            
              GRACEVILLE FOODMARKET
            
              gt ccell ceramic coils
            
              gt ccell coil
            
              gt mesh coil
            
              gti coils
            
              gtx coils
            
              gtx coils compatibility
            
              gtx coils near me
            
              gtx one coils
            
              IGA SPRINGFIELD LAKES
            
              IGA X-PRESS FOREST LAKE
            
              Innokin
            
              innokin ajax coil compatibility
            
              innokin ajax plex3d matrix coil 16ohm 5 pack
            
              innokin ajax tank
            
              INNOKIN COILS
            
              innokin endura t18 australia
            
              innokin endura t18 replacement coils 5 pack
            
              innokin mvp5 coils
            
              innokin prism t20 s replacement coil
            
              innokin replacement coils for ajax tank atomizer 5pcs
            
              innokin t18 replacement coils
            
              innokin t20s australia
            
              innokin t20s coils
            
              IPSWICH BREW CO
            
              JUNCTION NEWS ANNERLEY
            
              k pin mini coils
            
              kangertech 0.2 ohm coils
            
              kangertech 0.5 ohm coil
            
              kangertech australia
            
              kangertech clocc 1.5 coils
            
              kangertech clocc coils
            
              kangertech clocc coils 0.15
            
              kangertech clocc coils 0.5
            
              kangertech clocc coils 1 ohm
            
              kangertech coils australia
            
              kangertech ssocc coils australia
            
              kangertech subox mini australia
            
              KFB2
            
              KINKY KLOSETT
            
              m3
            
              mini accessories
            
              nautilus
            
              nikola antares coils
            
              nikola antares coils au
            
              nikola antares pod coils au
            
              nikola antares pod mod
            
              nikola antares replacement coils
            
              nrg gt core coil
            
              nunchaku coils
            
              obs
            
              orca solo
            
              orca solo coils
            
              orca solo replacement coils
            
              OXFORD STREET NEWS
            
              pnp coil
            
              prism s coil
            
              prism s replacement coil
            
              prism t20s
            
              qf strip coil
            
              smok
            
              smok 0.15 coil 40 80w best 60 70w
            
              smok 0.15 mesh coil
            
              smok 0.15 ohm 40 80 watts baby v8
            
              smok 0.15 ohm 40 80w
            
              smok coil
            
              smok coil 0.4 40 80w
            
              smok coils
            
              smok coils australia
            
              smok m17 coils near me
            
              smok m17 compatible coils
            
              smok priv m17 coil replacement
            
              smok stick aio coils near me
            
              smok stick aio compatible coils
            
              smok stick m17 coil compatibility
            
              smok stick m17 spare parts
            
              smok stick prince v12 coil
            
              smok stick replacement coils
            
              smok v12 prince coil
            
              smok v8 baby mesh coil amazon
            
              smok v8 t12 coil
            
              THRIFTY FOOD MARKET BETHANIA
            
              uwell
            
              uwell caliburn g coils
            
              uwell caliburn g replacement coil
            
              uwell crown
            
              uwell valyrian ii
            
              v12 prince coil compatibility
            
              v8 baby q2 0.4 dual coils
            
              v8 baby q2 coils
            
              v8 baby q4 coil
            
              v8 baby strip coil
            
              v8 baby strip replacement coils 0.15 ohm 5 pack
            
              vape king coils
            
              vape pen coils
            
              vape station
            
              vapefly optima coils
            
              vaporesso
            
              vaporesso ccell ceramic coils
            
              vaporesso ccell-gd ceramic replacement coils
            
              vaporesso ceramic coils
            
              vaporesso coils
            
              vaporesso euc coil review
            
              vaporesso euc coils 5 pack
            
              vaporesso euc coils australia
            
              vaporesso gt ccell 0 5 ohm coils
            
              vaporesso gt ccell ceramic coils
            
              vaporesso gt ccell coils review
            
              vaporesso gt coils australia
            
              vaporesso gt compatible coils
            
              vaporesso gtx coils compatibility
            
              vaporesso gtx one replacement coils
            
              vaporesso qf coils - skrr tank
            
              vaporesso qf coils australia
            
              vaporesso qf coils compatibility
            
              vaporesso skrr coils
            
              vaporesso skrr qf coils
            
              vaporesso skrr qf replacement coils
            
              vaporesso skrr qf vape coils
            
              vaporesso target mini ccell-gd ceramic coils for target tank
            
              vapour coils
            
              voopoo
            
              voopoo pnp coils
            
              voopoo pnp coils australia
            
              voopoo tpp
            
              voopoo tpp coil
            
              vsmajor
            
              vsmini
            
              x
            
              z coil
            
              zeus coil
            
          
        
      
      
    
  
  
    
      
        
          
          Filter by
          
        
        
  
  
  

      
      
        
          
            
              
                Filter by
                Showing 47 of 47 products
              
            
            
                  
                    
                      
                        Availability
                        
                        
                      
                    
                    
                      
                        Availability 
                      
                      
                            
                              
                              
                              In stock
                              42
                            
                          
                            
                              
                              
                              Out of stock
                              19
                            
                          

                      
                        Clear
                        Apply
                        &lt;button class=&quot;btn button button--primary&quot;>Apply&lt;/button>
                      
                    
                  
                

                  
                    
                      
                        Price
                        
                        
                      
                    
                    
                      
                        Price
                      

                      The highest price is $39.95

                      
                        
                          
                            From 
                          
                          
                            $
                            
                            
                          
                        
                        
                          
                            To 
                          
                          
                            $
                            
                            
                          
                        
                      
                    
                  
                

                  
                    
                      
                        FeaturedBest sellingAlphabetically, A-ZAlphabetically, Z-APrice, low to highPrice, high to lowDate, old to newDate, new to old
                      
                    
                  
                
                Clear
                Apply
                &lt;button class=&quot;btn button button--primary&quot;>Apply&lt;/button>
              
            

            
          
        
      
    
  

    
    
    
      
        
          
          
            
              
                
                
                
                
            
            
              
                
                  FeaturedBest sellingAlphabetically, A-ZAlphabetically, Z-APrice, low to highPrice, high to lowDate, old to newDate, new to old
                  
                
              
            
          
        
      
      
      
              
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/geekvape-zeus-replacement-coils-vape-station-586440_1024x.jpg?v=1663122428&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape Zeus coil (Z COIL)
                          
                          
                            

  
    $22.90
  


                          
                          Buy Geekvape Zeus Coil at Vape Station Redbank Plains.
The Geekvape Zeus Coil is best compatible with Geekvape Zeus Tank/Geekvape Aegis Legend Zeus Kit. 5 pieces each pack. Made of KA1, the Z Coil is a meshed coil for a longer ...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/vaporesso-gt-core-coils-3pk-default-vape-station-229145_1024x.jpg?v=1614226787&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Vaporesso GT Core Coil 3pk
                          
                          
                            

  
    $15.90
  


                          
                          Buy Vaporesso GT Core Coil at Vape Station Redbank Plains. 
Vaporesso GT Core Coil in 3packs is designed for NRG/ NRG SE tanks. We provide GT4, GT4 Meshed, GT8, GT CCELL, GT CCELL2, and GT Meshed coils for your selection. 
Para...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/euc-coils-ceramic-mesh-and-traditional-5-pack-accessories-vape-station-brown-ceramic-06-ohm-804550_1024x.jpg?v=1614226806&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                











    
        
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
            
            
                
                
                
                
                    
                
            
        
    



                              
                            
                          
                          
                            
                              
                            
                          
                          
                            EUC coils ceramic, mesh and traditional (5 pack)
                          
                          
                            

  
    $17.95
  


                          
                          Product Introduction
5pcs Vaporesso Traditional/mesh and traditional EUC spare coil. Suitable for Veco Solo Plus 
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/aegis-p-series-coil-vape-station-716743_1024x.jpg?v=1663121557&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape P Coil
                          
                          
                            

  
    $21.90
  


                          
                          Buy the Geekvape P Coil at Vape Station Redbank Plains. 
It is best compatible with GeekVape Aegis Boost Pro Pod/GeekVape Aegis Boost Pro Pod Kit/GeekVape Aegis Boost Pro Empty Pod. And the Geekvape P Series Coil comes with 0.2...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/aegis-boost-mesh-coil-pack-5pk-default-vape-station-669366_1024x.jpg?v=1663120544&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Geekvape B COIL 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy Geekvape B Coil 5pcs at Vape Station Redbank Plains. 
The Geekvape Aegis Boost Replacement Coil is specially designed for Geekvape Aegis Boost Kit, Geekvape Aegis Boost Plus kit, Geekvape Aegis Boost Plus Pod Cartridge, and...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/caliburn-g-replacement-coil-vape-station-08ohm-265348_1024x.jpg?v=1663127373&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Uwell Caliburn G coil
                          
                          
                            

  
    $22.95
  


                          
                          Buy the Uwell Caliburn G coils at the Vape Station Australia store at Redbank Plains. They are compatible with the Caliburn G Pod Starter Kit only.
Specs:

UN2-Meshed H 0.8ohm: 13 - 18W (smoother airflow) 
FeCrAl 1.0ohm: (tight...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/supermesh-x1-coil-pack-5pk-default-vape-station-ka1-02ohm-103744_1024x.jpg?v=1663121933&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                Accessories
                              
                            
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            Geekvape Supermesh Coil 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy GeekVape SuperMesh Coil at Vape Station.
Here comes the GeekVape SuperMesh coil designed for Shield tank, Aero tank, Aero Mesh tank and Cerberus subohm tank. 0.4ohm IM1 coil, 0.15ohm IM4 coil, 0.2ohm Mesh Coil are included ...
                          
  

    
      
      
        
          
            
          
        
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/drag-x-pnp-repacement-coils-vape-station-695680_1024x.jpg?v=1663123068&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            VOOPOO PNP Coil 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Buy Voopoo PNP Coil at Vape Station Redbank Plains. The VOOPOO PNP Coil is a versatile coil which is specially designed for the VOOPOO DRAG Baby Trio starter kit but is also used for many other Voopoo kits like the VOOPOO VINCI...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/orca-solo-replacement-coils-accessories-vape-station-334157_1024x.jpg?v=1601425725&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            Orca solo replacement coils 5pk
                          
                          
                            

  
    $19.95
  


                          
                          Orca solo replacement coils for orca solo 5pk
 
&quot;Keyword&quot;&quot;orca solo replacement coils 5pk&quot;&quot;orca solo coils&quot;&quot;orca solo&quot;
                          
  

    
      
      
        
          
            
          
        
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/gtx-coils-default-vape-station-gtx-mesh-03ohm-327879_1024x.jpg?v=1663118702&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            Vaporesso GTX COIL 5pk
                          
                          
                            

  
    $24.95
  


                          
                          Let&quot; , &quot;'&quot; , &quot;s welcome the Vaporesso GTX Coil.  Vaporesso TARGET PM80 GTX Coil. is specially designed for Vaporesso Target PM80 Kit, Vaporesso Target PM80 Empty Pod, Vaporesso GTX One Kit, and Vaporesso GTX Tank 18. And the TARGET PM80 ...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/voopoo-tpp-mesh-coil-3pcs-vape-station-379866_1024x.jpg?v=1663123825&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                              
                              
                                
                                  
                          
                        
                      
                      
                        
                          
                            
                              
                                COILS
                              
                            
                              
                            
                              
                            
                          
                          
                            
                              
                                










    


                              
                            
                          
                          
                            
                              
                            
                          
                          
                            VOOPOO TPP Coil 3pcs
                          
                          
                            

  
    $17.95
  


                          
                          Buy Voopoo TPP Coil at Vape Station Redbank Plains store.
Take the VOOPOO TPP Coil for your VOOPOO TPP Pod Tank, VOOPOO DRAG X Plus Kit, VOOPOO DRAG 3 Kit. The TPP series coils feature a new patented technology that not only in...
                          
  

    
      
      
        
          
         
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
                    
                      

                        
                          
                            
                            
                              &lt;img class=&quot;img-fluid product__thumbnail-second lazyload&quot; data-src=&quot;//cdn.shopify.com/s/files/1/2663/3040/products/innokin-t18-replacement-coil-pack-5-default-vape-station-354944_1024x.jpg?v=1616564677&quot; alt=&quot;&quot;>
                            
                            
                            
                              
                              
                            
                          
                        
                      
                      
                        
                          
                            
                              
                                COIL PACKS
                              
                            
                              
                            
                              
                            
                              
                            
                          
                          
                          
                            
                              
                            
                          
                          
                            INNOKIN - T18 Replacement Coil pack (5)
                          
                          
                            

  
    $17.95
  


                          
                          T18 replacement coil pack (5)
1.5 ohM horizontal coils. 
&quot;Keyword&quot;&quot;endura t18 coils australia&quot;&quot;endura t18 compatible coils&quot;&quot;innokin t18 replacement coils&quot;&quot;endura t18 coils near me&quot;&quot;innokin t18 replacement coils&quot;&quot;innokin endura ...
                          
  

    
      
        
      
      
    
  
  
    
	
		
	

  
  
    
  
    
  

  
  
    
  

                        
                      
                    
                  
  
  
  Showing 1 - 0 of 47 item(s)



  
    
      
        
        Previous
      
    
  

  
    
      
        1
      
    
  
    
      
        2
      
    
  
    
      
        3
      
    
  
    
      
        4
      
    
  

  
    
      
        Next
        
      
    
  



    
  



          
        

        

  
  
    
      
        
          
          
            
              
                
                  Contact Us
                  
                
              
              
                
                  
                    
                    Showroom: Shop 7, 319-321 Redbank Plains Road, Redbank Plains, QLD Australia
                  
                
                
                  
                    
                    07 3418 4513 OR 1300 355 535
                  
                
                
                  
                    
                    info@vapestation.co
                  
                
                
                  
                    
                    Opening Hours: MON - FRI - 10:00 am - 6:00 pm SAT - 10:00 am - 4:00pm  SUN - CLOSED
                  
                
              
            
          
        
          

              
                
                  USEFUL LINKS
                  
                
              
              
                
                  
                    
                      
                        GENERAL INFORMATION
                      
                    
                      
                        ZIP PAY
                      
                    
                      
                        PRIVACY POLICY
                      
                    
                      
                        REFUND POLICY
                      
                    
                      
                        TERMS OF SERVICE
                      
                    
                      
                        VAPE SAFETY
                      
                    
                  
                
              
            
          
          
        
          

              
                
                  Store Locator
                  
                
              
              
                
                  
                    
                      
                        
                      
                    
                  
                
              
            
          
          
         
      
    
  

  
    
      
        
          Copyright © 2022, VAPE STATION | Powered By Shopify | Developed by Bibidh Subedi and Eclogy.com
        
        
          
            
              
            
          
        
      
    
  



      
    &quot;))]</value>
      <webElementGuid>98a3f25e-0a90-4887-95c0-5f9da1461854</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
