<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Ishimwe Laetitia                      _7ed513</name>
   <tag></tag>
   <elementGuidId>bf2a56ef-f46c-419f-b6bc-ca1aed36f976</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        





































































    
        
            
                    
                        

                    

            
            

                         Ishimwe, Laetitia 
                        
                            
                        
                        
                        
                Version 4.11.0-beta.62

            
            
                


    /*.red-tooltip + .tooltip > .tooltip-inner {
        background-color: #f00;
        width: 600px;
    }*/
    .tooltip_templates {
        display: none;
    }
    /* Dropdown Button */
    .dropbtn {
        background-color: #08497f;
        color: white;
        padding: 2px 16px 0px 16px; 
        font-size: 16px;
        border: none;
    }

    /* The container &lt;div> - needed to position the dropdown content */
    .dropdown {
        position: relative;
        display: inline-block;
    }

    /* Dropdown Content (Hidden by Default) */
    .dropdown-content {
        display: none;
        position: absolute;
        background-color: #EBF9FF;
        min-width: 160px;
        box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
        z-index: 1;
    }

    /* Links inside the dropdown */
    .dropdown-content a {
        color: black;
        padding: 12px 10px;
        text-decoration: none;
        display: block;
    }

    /* Change color of dropdown links on hover */
        .dropdown-content a:hover {
            background-color: #08497f;
            color: white;
        }

    /* Show the dropdown menu on hover */
    .dropdown:hover .dropdown-content {display: block;}

    /* Change the background color of the dropdown button when the dropdown content is shown */
    .dropdown:hover .dropbtn {
        background-color: #EBF9FF;
        color: black;
    }





    
        
        
            English
            French
            German
            Spanish
        
    

        
            
        


    
        
    


    var started = $('#hdnStarted').val();
    if (started == 1) {
        $('#lngDropdwn').css('display', 'none');
    }
    //debugger;
    //$('#patientImg').tooltipster('content', $(&quot;#tooltip_content&quot;).html());
    
    $('#patientImg').tooltipster({
        theme: 'tooltipster-noir'
    });






            
        


    





    .token-input-token-facebook {
        font-weight: bold;
        background-color: #08497f;
        color: white;
    }

    .token-input-list-facebook {
        width: 100%;
    }










    
        

    

    
        
            
        
        
            
                
            
            
                Male / 31 (12/12/1988)
            
            
                
                    Kigali, Kigali City | 11:46 AM
                
            
        
        
            
                

            
        
    



 
/*Menu Patient */

    var menuPatientLoaded = true;

    $(document).ready(function () {
        $('#patientImg').tooltipster({
            theme: 'tooltipster-noir'
        });

        $('#hdLocation').tooltipster({
            theme: 'default',
            animation: 'fade',
            content: ''
        });


        $('#memName').tooltipster({
            theme: 'default',
            animation: 'fade',
            content: ''
        });

        UpdateMemberMenuPatient();

        UpdateDobGenderMenuPatient();

        UpdateLocationMenuPatient();

        UpdateTimeMenuPatient();

    });



    
        
                
                    
                        
                            
                        
                        
                            Demographics
                        
                    
                
                            
                    
                        
                            
                        
                        
                            
                                    
                                        Location
                                    


                            
                        
                    
                
                            
                    
                        
                            
                        
                        
                            Symptoms
                        
                    
                
                                        
                    
                        
                            
                        
                        
                            Related
                        
                    
                
                            
                    
                        
                            
                        
                        
                            Interview
                        
                    
                
                            
                    
                        
                            
                        
                        
                            
                                    
                                        Coordinate
                                    
                            
                        
                    

                
        
    
    

    
    
        
            Contacts
        

        
                
                    
                
        

    
    
        
            Action
        
        
            Type
        
        
            Name
        
        
            Phone
        
       
    
    

        
            
                
                
            

        
    



    var deleteContact = 'Delete Contact';
    var deleteContQuestion = 'Are you sure you want to delete the contact?';
    var message = '';
    var cantaddPatient = 'You can&amp;#x27;t insert another patient. Please verify.';
    var contacts = 'Contacts';


    if (message != '') {
        $.confirm({
            title: contacts,
            content: message,
            buttons: {
                Yes: {
                    text: yes,
                    btnClass: 'btn-info',
                    action: function () {
                        return;
                    }
                }

            }
        });
    }
    //$('.contactType').tooltipster({
    //    theme: 'tooltipster-noir',
    //    contentAsHTML: true
    //});

    
        
    



    $(document).ready(function() {
        LoadContacts('');
    });


        
            
                
                    
                        
                            
                                
                                    Symptoms
                                
                            
                            
                                
                                    
                                
                            

                            
                                    
        
            
            Common Symptoms
        
    

        
            +
            Abdominal guarding
        
        
            +
            Abdominal pain, migrating to the right lower quadrant
        
        
            +
            Abnormal foveal avascular zone
        
        
            +
            Abnormal or loss of taste
        
        
            +
            Abnormally deep, labored and gasping breathing
        
        
            +
            Absence of deep reflexes
        
        
            +
            Accelerated breathing
        
        
            +
            Achilles tendon hyperreflexia
        
        
            +
            Acidic taste in mouth
        
        
            +
            Active wound bleeding
        
        
            +
            Activity related to sex despite negative consequences
        
        
            +
            Acute thigh or buttock pain appearing during exertion and relieving after short rest
        
        
            +
            Adenoid face
        
        
            +
            Age &lt; 18
        
        
            +
            Age 45-55
        
        
            +
            Age above 40
        
        
            +
            Age above 60
        
        
            +
            Agitation
        
        
            +
            Ailments occurring when reducing alcohol consumption
        
        
            +
            Alcohol-related amnesia
        
        
            +
            Allelomimetic behavior, allomimetic behavior
        
        
            +
            Allen-Cleckley sign
        
        
            +
            Anemia
        
        
            +
            Anicteric presentation
        
        
            +
            Animal bite
        
        
            +
            Animal bites
        
        
            +
            Ankle injury
        
        
            +
            Ankle pain
        
        
            +
            Ankle pain while moving
        
        
            +
            Anorectal pain
        
        
            +
            Anticoagulant use
        
        
            +
            Anxiety
        
        
            +
            Anxiety attack
        
        
            +
            Aponeurotic reflex
        
        
            +
            Appendix removal in the past
        
        
            +
            Appetite for salty foods
        
        
            +
            Apraxia
        
        
            +
            Arm injury
        
        
            +
            Ascending muscle weakness
        
        
            +
            Ascending paresthesias
        
        
            +
            Ascites
        
        
            +
            Asymmetric pupils
        
        
            +
            Asymmetrical neck
        
        
            +
            Asymmetry of a skin mole or birthmark
        
        
            +
            At least six light brown marks on skin
        
        
            +
            Atherosclerosis
        
        
            +
            Atmospheric pressure changes
        
        
            +
            Atonic bladder
        
        
            +
            Auditory hallucinations
        
        
            +
            Auditory verbal agnosia
        
        
            +
            Auscultative fine crackles over the thorax
        
        
            +
            Auscultative rhonchi over the thorax
        
        
            +
            Auscultative wheezes over the thorax
        
        
            +
            Avoiding actions, places, or people that bring back memories of trauma
        
        
            +
            Avoiding eating and drinking in public
        
        
            +
            Avoiding eating in someone's presence
        
        
            +
            Avoiding public appearances
        
        
            +
            Avoiding talking to authority figures
        
        
            +
            Avoiding thoughts, feelings, conversations related to traumatic experience
        
        
            +
            Babinski sign
        
        
            +
            Back injury
        
        
            +
            Back pain
        
        
            +
            Back pain decreases during rest
        
        
            +
            Back pain lasting several hours
        
        
            +
            Back pain radiating through the back of thigh to knee
        
        
            +
            Back pain that comes and goes
        
        
            +
            Back pain worsening after physical exertion
        
        
            +
            Back pain worsens by touching or pressing the sore area
        
        
            +
            Bad breath
        
        
            +
            Behavioral changes, ignoring consequences and social norms
        
        
            +
            Belching
        
        
            +
            Belly pain spreading to the left shoulder or shoulder blade
        
        
            +
            Belly pain spreading to the lower or middle back
        
        
            +
            Belly pain spreading to the right shoulder area
        
        
            +
            Big toe pain
        
        
            +
            Bilateral absence of Achilles tendon reflex
        
        
            +
            Bilateral limb apraxia
        
        
            +
            Bilateral muscle paralysis
        
        
            +
            Bing sign
        
        
            +
            Binge-eating attacks
        
        
            +
            Bizarre delusion
        
        
            +
            Black spots in vision
        
        
            +
            Black stools
        
        
            +
            Blackheads
        
        
            +
            Bladder not emptying completely
        
        
            +
            Bladder sphincter dysfunction
        
        
            +
            Bleeding a lot recently
        
        
            +
            Bleeding from anus
        
        
            +
            Bleeding from ear
        
        
            +
            Bleeding from nose
        
        
            +
            Bleeding gums
        
        
            +
            Bleeding into joints
        
        
            +
            Bleeding into muscles
        
        
            +
            Blisters around mouth or ear
        
        
            +
            Blisters on the skin or on the mucous membrane
        
        
            +
            Bloating
        
        
            +
            Blood in stool
        
        
            +
            Blood in urine
        
        
            +
            Blood pressure over 180/120 mmHg
        
        
            +
            Bloodshot eye
        
        
            +
            Bluish-gray colored skin
        
        
            +
            Blunted affect
        
        
            +
            BMI below 19
        
        
            +
            Body aches
        
        
            +
            Body tingling
        
        
            +
            Bone deformation
        
        
            +
            Bone pain
        
        
            +
            Bone pain lasting less than a week
        
        
            +
            Bone pain lasting longer than a week
        
        
            +
            Bowlegs
        
        
            +
            Breathing problems after injury
        
        
            +
            Brick-colored spots resembling leopard spots
        
        
            +
            Broken bone
        
        
            +
            Broken bone without having had an accident
        
        
            +
            Broken hair
        
        
            +
            Brown nail discoloration
        
        
            +
            Brudzinski sign
        
        
            +
            Bruise after trauma
        
        
            +
            Bruising around the eyes
        
        
            +
            Buccofacial or orofacial apraxia
        
        
            +
            Burn
        
        
            +
            Burn involving face, hands, feet, genitalia, or major joints
        
        
            +
            Burning chest pain
        
        
            +
            Burning or gnawing stomach pain
        
        
            +
            Burning skin
        
        
            +
            Burning skin on face
        
        
            +
            Burning tongue
        
        
            +
            Buttocks pain
        
        
            +
            Calcification of the tympanic membrane
        
        
            +
            Calf injury
        
        
            +
            Calf pain
        
        
            +
            Cannabis, marijuana smoking
        
        
            +
            Carbon monoxide source
        
        
            +
            Cardiac murmurs
        
        
            +
            Cavities
        
        
            +
            Ceasing or limiting alcohol consumption
        
        
            +
            Cervical artery murmur
        
        
            +
            Cervical dystonia
        
        
            +
            Chaddock sign
        
        
            +
            Change in bowel habits
        
        
            +
            Change in color of a skin mole or birthmark
        
        
            +
            Change in color of a tooth
        
        
            +
            Change in handwriting
        
        
            +
            Change in personality
        
        
            +
            Change in shape of a skin mole or birthmark
        
        
            +
            Chemical or electrical burn
        
        
            +
            Chemosis
        
        
            +
            Chest injury
        
        
            +
            Chest pain
        
        
            +
            Chest pain behind breastbone
        
        
            +
            Chest pain continuing after rest
        
        
            +
            Chest pain lasting between 30 minutes and 8 hours
        
        
            +
            Chest pain lasting less than 30 minutes
        
        
            +
            Chest pain lasting over 8 hours
        
        
            +
            Chest pain on exertion
        
        
            +
            Chest pain radiating between shoulder blades
        
        
            +
            Chest pain radiating to neck
        
        
            +
            Chest pain subsiding during rest
        
        
            +
            Chest pain worsened by stress
        
        
            +
            Chest pain worsening during chest wall movement
        
        
            +
            Chest pain worsening on palpation of costochondral joints
        
        
            +
            Chest pain worsening when lying down
        
        
            +
            Chest pain worsening when touching or pressing
        
        
            +
            Chest pain worsening with deep breath or cough
        
        
            +
            Chest pain, radiating to left upper limb
        
        
            +
            Cheyne-Stokes respiration
        
        
            +
            Chills
        
        
            +
            Chorea
        
        
            +
            Chronic constipation
        
        
            +
            Chronic heavy alcohol use
        
        
            +
            Chronic shortness of breath
        
        
            +
            Chronic stuffy nose
        
        
            +
            Chronic UV exposure
        
        
            +
            Chvostek sign
        
        
            +
            Cigarette smoking during illness
        
        
            +
            Clasp-knife spasticity
        
        
            +
            Clicking sounds in the jaw joint when opening or closing the mouth
        
        
            +
            Clogged ear
        
        
            +
            Close family or friends pointing out excessive drinking
        
        
            +
            Cloudy eye
        
        
            +
            Club-foot
        
        
            +
            Coating on tongue
        
        
            +
            Cognitive disturbances
        
        
            +
            Cogwheel muscle rigidity
        
        
            +
            Cold and clammy skin
        
        
            +
            Cold feet and toes
        
        
            +
            Cold foods or drinks ease toothache
        
        
            +
            Cold hands or fingers
        
        
            +
            Cold sensitivity
        
        
            +
            Colic stomach pain
        
        
            +
            Collection of pus in the front of the eye
        
        
            +
            Color vision deficiency
        
        
            +
            Coma
        
        
            +
            Complete vision loss
        
        
            +
            Compromised immune system
        
        
            +
            Confabulation
        
        
            +
            Confusion after injury
        
        
            +
            Confusion or disorientation
        
        
            +
            Constant muscle stiffness in a limb
        
        
            +
            Constant need for sex
        
        
            +
            Constant urge to drink alcohol
        
        
            +
            Constipation
        
        
            +
            Constriction of the pupil
        
        
            +
            Contact lenses
        
        
            +
            Contact with allergen
        
        
            +
            Contact with animals
        
        
            +
            Contact with animals, cat
        
        
            +
            Contact with insecticides
        
        
            +
            Contact with sick person, chickenpox
        
        
            +
            Contact with weed killers
        
        
            +
            Contentiousness
        
        
            +
            Corn or callus
        
        
            +
            Corneal reflex weak or absent
        
        
            +
            Cornell's sign
        
        
            +
            Coronary disease
        
        
            +
            Corticosteroid use
        
        
            +
            Cough
        
        
            +
            Cough lasting less than three weeks
        
        
            +
            Cough lasting more than eight weeks
        
        
            +
            Cough, lasting three to eight weeks
        
        
            +
            Coughing up blood
        
        
            +
            Coughing up phlegm
        
        
            +
            Coughing up phlegm in the morning
        
        
            +
            Coughing up pink phlegm
        
        
            +
            Coughing up thick yellow or green phlegm
        
        
            +
            Cracked corners of lips
        
        
            +
            Cramps in calves
        
        
            +
            Crampy stomach pain
        
        
            +
            Creaking of joint during movement
        
        
            +
            Crossed eyes
        
        
            +
            Crowded teeth
        
        
            +
            Current cigarette smoking
        
        
            +
            Cystoid macular edema
        
        
            +
            Damage to 2 separate areas of innervation
        
        
            +
            Dance's sign
        
        
            +
            Dandruff
        
        
            +
            Darier sign
        
        
            +
            Dark circles under eyes
        
        
            +
            Dark or bluish hard mass in the groin
        
        
            +
            Dark rings appearing around the iris of the eye
        
        
            +
            Dark thoughts
        
        
            +
            Dark urine
        
        
            +
            Darkening of the skin
        
        
            +
            Dark-red spots in optic disc vessels
        
        
            +
            Daytime swelling
        
        
            +
            Decline in tongue size
        
        
            +
            Decrease of groin mass size while lying down
        
        
            +
            Decreased bowel sounds
        
        
            +
            Decreased deep tendon reflexes
        
        
            +
            Decreased hearing
        
        
            +
            Decreased intensity of breath sounds
        
        
            +
            Decreased intensity of breath sounds, right basilar lung zones
        
        
            +
            Decreased motivation
        
        
            +
            Decreased need for sleep
        
        
            +
            Decreased pupillary light reflex
        
        
            +
            Decreased skin elasticity
        
        
            +
            Decreased vision
        
        
            +
            Deep and dull muscle pain
        
        
            +
            Deep filling of treated tooth
        
        
            +
            Deepened spaces between the gums and the teeth
        
        
            +
            Deformed joint
        
        
            +
            Deformed joints after injury
        
        
            +
            Degeneration of toenail
        
        
            +
            Delusion of guilt or sin
        
        
            +
            Delusion of impoverishment
        
        
            +
            Delusions
        
        
            +
            Delusions of grandeur
        
        
            +
            Delusions of influence
        
        
            +
            Delusions of persecution
        
        
            +
            Delusions of reference
        
        
            +
            Delusions of thought insertion, thought withdrawal, thought broadcasting
        
        
            +
            Demarcation of retinal detachment (pigmented or nonpigmented)
        
        
            +
            Dennie-Morgan fold
        
        
            +
            Dental plaque
        
        
            +
            Dental trauma
        
        
            +
            Depressed mood
        
        
            +
            Dermatological changes located in the mouth
        
        
            +
            Dermatological changes, male genital area
        
        
            +
            Destructive thoughts towards other people or things
        
        
            +
            Diabetes
        
        
            +
            Diagnosed aortic aneurysm
        
        
            +
            Diagnosed asthma
        
        
            +
            Diagnosed chronic obstructive pulmonary disease
        
        
            +
            Diagnosed peripheral vascular disease
        
        
            +
            Diagnosed psoriasis
        
        
            +
            Diarrhea
        
        
            +
            Diarrhea looking like rice water
        
        
            +
            Diarrhea mainly at night
        
        
            +
            Diarrhea with more than six stools a day
        
        
            +
            Diarrhea, lasting more than 14 days
        
        
            +
            Diarrhea, lasting up to 14 days
        
        
            +
            Difference in size of testicles
        
        
            +
            Difficulty achieving orgasm
        
        
            +
            Difficulty bending down
        
        
            +
            Difficulty biting and chewing
        
        
            +
            Difficulty bonding with infant
        
        
            +
            Difficulty completely closing one eye
        
        
            +
            Difficulty determining which tooth aches
        
        
            +
            Difficulty finding everyday objects
        
        
            +
            Difficulty initiating a swallow
        
        
            +
            Difficulty learning new things
        
        
            +
            Difficulty moving eyes
        
        
            +
            Difficulty moving tongue
        
        
            +
            Difficulty moving upper limbs
        
        
            +
            Difficulty not smoking first cigarette of the day
        
        
            +
            Difficulty refraining from smoking cigarettes where not permitted
        
        
            +
            Difficulty retracting foreskin
        
        
            +
            Difficulty swallowing
        
        
            +
            Difficulty with mouth opening
        
        
            +
            Difficulty with speech
        
        
            +
            Dilated pupils
        
        
            +
            Diminished appetite
        
        
            +
            Diminished eye motility in the same direction
        
        
            +
            Diminished fear responses
        
        
            +
            Diminished superficial reflexes
        
        
            +
            Diminished testicles
        
        
            +
            Discharge from ear
        
        
            +
            Discharge from spaces between the gums and the teeth
        
        
            +
            Discharge from urethra
        
        
            +
            Discharge in tympanic cavity
        
        
            +
            Discharge under foreskin
        
        
            +
            Discharge when pressing penis
        
        
            +
            Diseases of joints
        
        
            +
            Disinhibition
        
        
            +
            Disorganized behavior
        
        
            +
            Disorientation or confusion lasting more than a few days
        
        
            +
            Disorientation or confusion lasting several hours or days
        
        
            +
            Dissociated sensory loss
        
        
            +
            Dix-Hallpike maneuver, rotatory nystagmus
        
        
            +
            Dizziness while turning the head
        
        
            +
            Dizziness, recurrent
        
        
            +
            Dizziness, vertigo
        
        
            +
            Dizzy
        
        
            +
            Double vision
        
        
            +
            Double vision resolving while tilting the head
        
        
            +
            Double vision while looking down and toward the nose
        
        
            +
            Double vision while looking in any direction
        
        
            +
            Double vision with images appearing one above the other or obliquely
        
        
            +
            Double vision with images appearing side by side
        
        
            +
            Drinking alcohol at work
        
        
            +
            Drinking alcohol despite awareness of harmful effects
        
        
            +
            Drinking alone
        
        
            +
            Drinking large quantities of alcohol
        
        
            +
            Drinking large quantities of alcohol at one time
        
        
            +
            Drinking to ease hangover
        
        
            +
            Drooping eyelids
        
        
            +
            Drooping lower eyelid
        
        
            +
            Drooping mouth corner
        
        
            +
            Drop attack
        
        
            +
            Dropping of the forefoot
        
        
            +
            Drunk driving
        
        
            +
            Dry cough
        
        
            +
            Dry discharge on eyelids
        
        
            +
            Dry eye
        
        
            +
            Dry mouth
        
        
            +
            Dry skin
        
        
            +
            Dry, cracked skin, bleeding occasionally
        
        
            +
            Dystonia
        
        
            +
            Ear injury
        
        
            +
            Earache
        
        
            +
            Easy bruising
        
        
            +
            Elbow injury
        
        
            +
            Elbow pain
        
        
            +
            Elbow pain while moving
        
        
            +
            Elbow swelling
        
        
            +
            Elevated mood
        
        
            +
            Enamel hypoplasia
        
        
            +
            Enlarged finger tips
        
        
            +
            Enlarged kidneys
        
        
            +
            Enlarged lymph nodes
        
        
            +
            Enlarged salivary glands
        
        
            +
            Enlarged spleen
        
        
            +
            Enlargement of hands, feet, or face
        
        
            +
            Enlargement of the lymphoid tissue in the pharynx
        
        
            +
            Epicritic sensation loss or impairment in fingertips
        
        
            +
            Erectile dysfunction
        
        
            +
            Excessive bowel sounds
        
        
            +
            Excessive fear in public situations
        
        
            +
            Excessive gambling
        
        
            +
            Excessive salivation
        
        
            +
            Excessive sweating
        
        
            +
            Expanding rash with migrating redness
        
        
            +
            Eye flashes
        
        
            +
            Eye pain
        
        
            +
            Eye rotation sideways and downward
        
        
            +
            Eye rotation upward and centrally
        
        
            +
            Eyelid twitching
        
        
            +
            Eyes sensitive to light
        
        
            +
            Face lacking expression or animation
        
        
            +
            Face numbness
        
        
            +
            Face swelling
        
        
            +
            Facial muscle spasms
        
        
            +
            Facial muscle weakness
        
        
            +
            Facial pain
        
        
            +
            Facial pain lasting longer than a couple of hours
        
        
            +
            Facial pain on one side
        
        
            +
            Facial pain triggered by eating, touching, or brushing teeth
        
        
            +
            Facial pain, stabbing
        
        
            +
            Facial weakness on one side of face
        
        
            +
            Fainting with loss of bladder or bowel control
        
        
            +
            Falling or losing gait on one side
        
        
            +
            Family history of bleeding disorders
        
        
            +
            Fast heartbeat
        
        
            +
            Fatigue
        
        
            +
            Fatigue more than six months
        
        
            +
            Fatty stools
        
        
            +
            Fear of certain environments where escape or getting help might be difficult
        
        
            +
            Fear of dying
        
        
            +
            Fear of going insane
        
        
            +
            Fear of losing control
        
        
            +
            Fear of obesity
        
        
            +
            Fear of object, situation, action
        
        
            +
            Fear of public appearances
        
        
            +
            Feeling detached from your body
        
        
            +
            Feeling dizzy or faint after standing up quickly
        
        
            +
            Feeling hot
        
        
            +
            Feeling of food getting stuck several seconds after swallowing
        
        
            +
            Feeling of guilt
        
        
            +
            Feeling of guilt or shame after drinking alcohol
        
        
            +
            Feeling of helplessness
        
        
            +
            Feeling of hopelessness
        
        
            +
            Feeling of incomplete defecation
        
        
            +
            Feeling of lump in throat
        
        
            +
            Feeling of rectal fullness
        
        
            +
            Feeling of resignation
        
        
            +
            Feeling of weight in scrotum
        
        
            +
            Feeling overweight despite low weight
        
        
            +
            Feeling pressure or pulsation inside the tooth
        
        
            +
            Feeling sick
        
        
            +
            Feeling unreal
        
        
            +
            Feet pain
        
        
            +
            Fever
        
        
            +
            Fever between 100.4 and 104 °F (38 and 40 °C)
        
        
            +
            Fever between 98.6 and 100.4°F (37 and 38°C)
        
        
            +
            Fever greater than 104 °F (40 °C)
        
        
            +
            Fever that comes and goes
        
        
            +
            Fever, temperature not measured
        
        
            +
            Financial recklessness
        
        
            +
            Fine downy hair
        
        
            +
            Finger flexion when tapping distal base of radius bone
        
        
            +
            Finger injury
        
        
            +
            Finger swelling
        
        
            +
            Fits of anger
        
        
            +
            Flaccid dysarthria
        
        
            +
            Flaccid paresis
        
        
            +
            Flaccid paresis, symmetrical
        
        
            +
            Flapping tremor
        
        
            +
            Flat affect
        
        
            +
            Foamy diarrhea
        
        
            +
            Foamy urine
        
        
            +
            Food coming back up into the mouth
        
        
            +
            Foot injury
        
        
            +
            Foot pain
        
        
            +
            Foot tingling or numbness
        
        
            +
            Forearm injury
        
        
            +
            Forearm pain
        
        
            +
            Forgetting names of close friends and family
        
        
            +
            Forward-leaning posture
        
        
            +
            Foul-smelling stools
        
        
            +
            Freckles under the armpits and groin
        
        
            +
            Frequent bowel movement
        
        
            +
            Frequent bronchitis
        
        
            +
            Frequent infections
        
        
            +
            Frequent runny nose, cough
        
        
            +
            Frequent urination
        
        
            +
            Frequent urination in large quantities
        
        
            +
            Frontal release signs
        
        
            +
            Full-thickness skin burn
        
        
            +
            Gallbladder removal in the past
        
        
            +
            Gambling
        
        
            +
            Gastric symptoms after eating or drinking foods containing lactose
        
        
            +
            General anxiety
        
        
            +
            Generalized headache
        
        
            +
            Genital injury
        
        
            +
            Genital itch
        
        
            +
            Glassy eyes
        
        
            +
            Goldflam sign
        
        
            +
            Gonda-Allen sign
        
        
            +
            Gordon sign
        
        
            +
            Grandiosity
        
        
            +
            Gray layer in throat
        
        
            +
            Groin mass impossible to reduce into abdomen manually
        
        
            +
            Groin mass possible to reduce into abdomen manually
        
        
            +
            Growths on genitals resembling cauliflower
        
        
            +
            Gum pain
        
        
            +
            Gustatory hallucinations
        
        
            +
            Gynecomastia
        
        
            +
            Hair loss
        
        
            +
            Hairy leukoplakia
        
        
            +
            Hallucinations
        
        
            +
            Halos around lights
        
        
            +
            Hand injury
        
        
            +
            Hand swelling
        
        
            +
            Hard lump in groin
        
        
            +
            Hard retinal exudates
        
        
            +
            Hawking
        
        
            +
            Head area injury
        
        
            +
            Head drop
        
        
            +
            Head pressure
        
        
            +
            Head tremors
        
        
            +
            Headache
        
        
            +
            Headache attacks lasting up to 5 minutes
        
        
            +
            Headache located around the forehead
        
        
            +
            Headache on one side
        
        
            +
            Headache reaching its climax within minutes
        
        
            +
            Headache worsened by stress
        
        
            +
            Headache worsened by tilting head forward
        
        
            +
            Headaches history longer than 3 months
        
        
            +
            Headaches history shorter than 3 months
        
        
            +
            Headaches lasting between 4 and 72 hours
        
        
            +
            Headaches lasting between five minutes and four hours
        
        
            +
            Headaches lasting from three to seven days
        
        
            +
            Hearing loss with variable intensity and duration
        
        
            +
            Hearing voices arguing
        
        
            +
            Hearing voices commenting
        
        
            +
            Heart enlargement
        
        
            +
            Heartburn
        
        
            +
            Heavy legs at the end of the day
        
        
            +
            Heel drop test
        
        
            +
            Hemianopsia bitemporalis
        
        
            +
            Hemiplegia
        
        
            +
            Hemophilia
        
        
            +
            Hepatomegaly
        
        
            +
            Hiccups
        
        
            +
            High cholesterol
        
        
            +
            High-arched palate
        
        
            +
            Hip injury
        
        
            +
            Hip swelling
        
        
            +
            History of anaphylactic reactions
        
        
            +
            History of cancer
        
        
            +
            History of deep vein thrombosis
        
        
            +
            History of depression episode
        
        
            +
            History of kidney stones
        
        
            +
            History of tonsillectomy
        
        
            +
            History of urinary tract infections
        
        
            +
            Hives
        
        
            +
            Hives after scratching
        
        
            +
            Hives appearing after exposure to cold, water, or wind
        
        
            +
            Hives appearing after exposure to heat
        
        
            +
            Hives appearing after exposure to prolonged pressure on the skin
        
        
            +
            Hives appearing after exposure to sunlight or UV radiation
        
        
            +
            Hoarseness
        
        
            +
            Hoarseness lasting more than two weeks
        
        
            +
            Honey-colored crust on the skin
        
        
            +
            Horizontal nystagmus
        
        
            +
            Hot flashes
        
        
            +
            Hyperalgesia
        
        
            +
            Hypertension
        
        
            +
            Hypertension, over 140 mmHg
        
        
            +
            Hypochondriacal delusions
        
        
            +
            Illusion of surroundings being bigger or smaller than they actually are
        
        
            +
            Impaired balance while walking
        
        
            +
            Impaired downward eye motion
        
        
            +
            Impaired lateral eye motion
        
        
            +
            Impaired medial eye motion
        
        
            +
            Impaired memory
        
        
            +
            Impaired short-term memory
        
        
            +
            Impaired smell
        
        
            +
            Impaired upward eye motion
        
        
            +
            Impaired vision
        
        
            +
            Impaired vision in one eye
        
        
            +
            Impaired vision on side with painful temple
        
        
            +
            Impulsive behavior
        
        
            +
            Inability to complete tasks
        
        
            +
            Inability to comprehend and formulate speech
        
        
            +
            Inability to enjoy life
        
        
            +
            Inability to find words
        
        
            +
            Inability to move mandible sideways
        
        
            +
            Inability to pass gas
        
        
            +
            Inability to produce or recognize musical sounds
        
        
            +
            Inability to recognize objects, people, sounds, shapes, or smells
        
        
            +
            Inability to remember important aspects of a traumatic experience
        
        
            +
            Inability to stand on one or both feet
        
        
            +
            Inability to stop drinking alcohol
        
        
            +
            Inability to stop flow of urine
        
        
            +
            Increase in sexual experiences or sensations needed to attain desired effect
        
        
            +
            Increased abdominal size
        
        
            +
            Increased appetite
        
        
            +
            Increased deep tendon reflexes
        
        
            +
            Increased expiratory phase
        
        
            +
            Increased libido
        
        
            +
            Increased muscle tension
        
        
            +
            Increased palatine reflex
        
        
            +
            Increased pharyngeal reflex
        
        
            +
            Increased thirst
        
        
            +
            Indigestion
        
        
            +
            Injury
        
        
            +
            Insect bite
        
        
            +
            Insomnia
        
        
            +
            Instituting angiotensin-converting enzyme inhibitor therapy
        
        
            +
            Intense stomach pain
        
        
            +
            Intense, non-committal relationships
        
        
            +
            Intention tremor
        
        
            +
            Intermittent urine flow
        
        
            +
            Interosseous muscles atrophy, feet
        
        
            +
            Interosseous muscles atrophy, hands
        
        
            +
            Intraretinal microvascular abnormalities
        
        
            +
            Iris neovascularization
        
        
            +
            Irregular heart rhythm lasting longer than 5 minutes
        
        
            +
            Irritability
        
        
            +
            Irritability when unable to engage in activity related to sex
        
        
            +
            Itching after a hot shower or bath
        
        
            +
            Itching aggravated by changes in temperature, sweat, or wearing wool
        
        
            +
            Itching and painful skin change on eyelid
        
        
            +
            Itching around anus
        
        
            +
            Itching in ear
        
        
            +
            Itching most intense at night
        
        
            +
            Itching of a foot or feet
        
        
            +
            Itching of eyes
        
        
            +
            Itching of skin
        
        
            +
            Itching of the scalp
        
        
            +
            Itchy throat or nose
        
        
            +
            Janeway lesion
        
        
            +
            Jaw jerk
        
        
            +
            Jaw pain
        
        
            +
            Joint pain
        
        
            +
            Joint pain in the morning
        
        
            +
            Joint pain while touching
        
        
            +
            Joint pain, aggravated during cold, damp weather
        
        
            +
            Joint pain, during hip movement
        
        
            +
            Joint pain, hip
        
        
            +
            Joint redness
        
        
            +
            Joint swelling
        
        
            +
            Kernig's sign
        
        
            +
            Knee injury
        
        
            +
            Knee pain
        
        
            +
            Knee pain while moving
        
        
            +
            Lack of boundaries in relationships
        
        
            +
            Lack of initiative, motivation, or drive
        
        
            +
            Lack of sweating
        
        
            +
            Lack of sweating on one side of face
        
        
            +
            Large blisters with diameter equal or greater than 5 mm
        
        
            +
            Last tetanus shot over 5 years ago
        
        
            +
            Left limb-kinetic apraxia
        
        
            +
            Leg injury
        
        
            +
            Leg pain occurring during exertion and relieved by rest
        
        
            +
            Leg weakness
        
        
            +
            Lengthening of reaction time
        
        
            +
            Levine's sign
        
        
            +
            Lhermitte's sign
        
        
            +
            Light or fair skin that easily burns
        
        
            +
            Lightening of the skin
        
        
            +
            Limb injury
        
        
            +
            Limb paralysis
        
        
            +
            Limited blood supply past the injury site
        
        
            +
            Limping due to pain
        
        
            +
            Lisch nodules
        
        
            +
            Local dark retinal blood vessels that lie in folds
        
        
            +
            Local elevation of retina
        
        
            +
            Local grey discoloration of retina
        
        
            +
            Local limb compression
        
        
            +
            Local retinal undulation
        
        
            +
            Long-term use of non-steroidal anti-inflammatory drugs
        
        
            +
            Loose teeth
        
        
            +
            Loss of body hair
        
        
            +
            Loss of bowel control
        
        
            +
            Loss of consciousness
        
        
            +
            Loss of feeling in both arms
        
        
            +
            Loss of feeling in face
        
        
            +
            Loss of feeling in only one arm or leg
        
        
            +
            Loss of feeling in part of ear
        
        
            +
            Loss of feeling in tongue
        
        
            +
            Loss of interest
        
        
            +
            Loss of interest in everyday matters
        
        
            +
            Loss of interest in sex
        
        
            +
            Loss of muscles
        
        
            +
            Loss of muscles in hands
        
        
            +
            Loss of sensation on one side of the face
        
        
            +
            Loss of sense of vibration in distal parts of extremities
        
        
            +
            Loss of taste buds
        
        
            +
            Loss of temperature sensation
        
        
            +
            Loss of vision in a superior quadrant
        
        
            +
            Loss of voice
        
        
            +
            Loss of writing ability
        
        
            +
            Loss or impairment of proprioception
        
        
            +
            Low blood pressure
        
        
            +
            Low sense of self-worth
        
        
            +
            Low urine output
        
        
            +
            Lower intraocular pressure (> 4-5mmHg) in affected eye compared to unaffected eye
        
        
            +
            Lower stomach pain
        
        
            +
            Lower-back pain
        
        
            +
            Lower-back pain radiating to the groin
        
        
            +
            Lower-back pain worsening in the night
        
        
            +
            Lump in abdomen
        
        
            +
            Lump in scrotum
        
        
            +
            Lying about eating, hiding food
        
        
            +
            Lying to hide the extent of your gambling
        
        
            +
            Major bleeding
        
        
            +
            Major force injury
        
        
            +
            Making rash or impulsive decisions
        
        
            +
            Male sexual immaturity
        
        
            +
            Malocclusions
        
        
            +
            Man
        
        
            +
            Many birthmarks
        
        
            +
            Marcus Gunn pupil
        
        
            +
            Measles rash
        
        
            +
            Mendel-Bechterev sign
        
        
            +
            Mental disturbances exacerbated by stress
        
        
            +
            Middle- or upper-back pain
        
        
            +
            Mild headache
        
        
            +
            Mild stomach pain
        
        
            +
            Minor uncontrollable bleeding
        
        
            +
            Moderate headache
        
        
            +
            Moderate stomach pain
        
        
            +
            Moniz sign
        
        
            +
            Mood better in the evening than in the morning
        
        
            +
            Mood swings
        
        
            +
            Morning headache
        
        
            +
            Morning stiffness of hand joints
        
        
            +
            Mottled skin
        
        
            +
            Mouth breathing
        
        
            +
            Mouth ulcers
        
        
            +
            Mucosal pallor
        
        
            +
            Mucous secretion in stool
        
        
            +
            Mucus dripping at the back of the throat
        
        
            +
            Multiple injuries
        
        
            +
            Muscle cramps
        
        
            +
            Muscle pain
        
        
            +
            Muscle pain triggered by increased stress
        
        
            +
            Muscle trembling
        
        
            +
            Muscle weakness
        
        
            +
            Muscle weakness mainly in upper arms or thighs
        
        
            +
            Muscle weakness, standing up or climbing stairs
        
        
            +
            Myringitis
        
        
            +
            Nail deformation
        
        
            +
            Nail injury
        
        
            +
            Narrow mouth, difficulty opening mouth
        
        
            +
            Nasal injury
        
        
            +
            Nasal voice
        
        
            +
            Neck injury
        
        
            +
            Neck pain
        
        
            +
            Neck pain appearing or worsening during head movement
        
        
            +
            Neck pain on one side
        
        
            +
            Need of higher alcohol quantities to get drunk
        
        
            +
            Need to smoke cigarette when waking up
        
        
            +
            Neglecting other duties and activities due to drinking
        
        
            +
            Neglecting other duties to pursue sex
        
        
            +
            Nervousness
        
        
            +
            Neural tenderness
        
        
            +
            New neurological changes, lasting over 24 hours
        
        
            +
            New neurological changes, since less than 24 hours
        
        
            +
            Night sweats
        
        
            +
            Nighttime cough
        
        
            +
            Nighttime swelling
        
        
            +
            Nihilistic delusions
        
        
            +
            No signs of upper respiratory infection
        
        
            +
            No skin injury
        
        
            +
            Non-healing wound
        
        
            +
            Non-serious eye injury
        
        
            +
            Nonstop drinking for several days
        
        
            +
            Normal body temperature
        
        
            +
            Normal lung sounds on auscultation
        
        
            +
            Normal pulse
        
        
            +
            Normal vital signs
        
        
            +
            Nose pain
        
        
            +
            Numbness in one lower limb
        
        
            +
            Numbness of both lower limbs
        
        
            +
            Numbness of both upper limbs
        
        
            +
            Numbness of limbs
        
        
            +
            Numbness of one upper limb
        
        
            +
            Numbness or tingling of sudden onset
        
        
            +
            Nystagmus
        
        
            +
            Obesity
        
        
            +
            Obsessive thoughts
        
        
            +
            Obsessive worries
        
        
            +
            Oculomotor nerve palsy
        
        
            +
            Oily skin
        
        
            +
            Olfactory hallucinations
        
        
            +
            One finger pain
        
        
            +
            One-sided sore throat
        
        
            +
            One-sided throat swelling
        
        
            +
            Opacified tympanic membrane
        
        
            +
            Opioid use
        
        
            +
            Oppenheim sign
        
        
            +
            Optic disc neovascularization
        
        
            +
            Optic nerve disc pallor
        
        
            +
            Orbicularis oris muscle spasm
        
        
            +
            Organic cause
        
        
            +
            Osler nodes
        
        
            +
            Overbite
        
        
            +
            Oversensitivity to smells
        
        
            +
            Oversensitivity to sound
        
        
            +
            Pain and redness in the area of the temple
        
        
            +
            Pain behind ear
        
        
            +
            Pain caused by firm pressure on tibia
        
        
            +
            Pain in back of head
        
        
            +
            Pain in both hands
        
        
            +
            Pain in both lower limbs
        
        
            +
            Pain in both lower limbs, stocking distribution
        
        
            +
            Pain in both upper limbs
        
        
            +
            Pain in crotch
        
        
            +
            Pain in fingers or hand
        
        
            +
            Pain in groin during coughing, sneezing or laughing
        
        
            +
            Pain in left lower part of stomach
        
        
            +
            Pain in left part of abdomen
        
        
            +
            Pain in lower limb
        
        
            +
            Pain in right lower part of your stomach
        
        
            +
            Pain in right part of abdomen
        
        
            +
            Pain in scrotum
        
        
            +
            Pain in the back of foot while standing on toes
        
        
            +
            Pain in the back of the leg, above the heel
        
        
            +
            Pain in the center of stomach
        
        
            +
            Pain in the jaw while yawning, chewing, or biting
        
        
            +
            Pain in the temples
        
        
            +
            Pain in upper limb
        
        
            +
            Pain increases when touching ear area
        
        
            +
            Pain near eye socket
        
        
            +
            Pain of the head of the penis
        
        
            +
            Pain of the skin around the nail
        
        
            +
            Pain when pressing abdomen
        
        
            +
            Pain when pressing abdomen in right upper part
        
        
            +
            Pain when pressing left upper part of abdomen
        
        
            +
            Pain when pressing stomach in lower part
        
        
            +
            Pain while urinating
        
        
            +
            Painful bowel movements
        
        
            +
            Painful ejaculation
        
        
            +
            Painful hard mass in groin
        
        
            +
            Painful lymph nodes
        
        
            +
            Painful skin change
        
        
            +
            Painful swallowing
        
        
            +
            Painless genital ulcer
        
        
            +
            Painless wound
        
        
            +
            Pale and brittle nails
        
        
            +
            Pale conjunctiva
        
        
            +
            Pale or clay-colored stools
        
        
            +
            Pale or grayish-blue fingers
        
        
            +
            Pale or grayish-blue fingers after exposure to stress or cold
        
        
            +
            Pale skin
        
        
            +
            Palpable neck mass
        
        
            +
            Palpitations
        
        
            +
            Palpitations, recurrent
        
        
            +
            Panic attacks in public appearances
        
        
            +
            Papilledema
        
        
            +
            Paracetamol overdose
        
        
            +
            Paralogical thinking
        
        
            +
            Paraspinal muscle tenderness
        
        
            +
            Paresthesia in both upper limbs, glove distribution
        
        
            +
            Parotid swelling
        
        
            +
            Partial limbs paralysis
        
        
            +
            Pelvis injury
        
        
            +
            Pencil-thin stools
        
        
            +
            Penile itching
        
        
            +
            Performing certain actions repeatedly to ease anxiety
        
        
            +
            Pericardial friction rub
        
        
            +
            Periodic facial pain
        
        
            +
            Perseveration
        
        
            +
            Persistent depressed mood
        
        
            +
            Phobia
        
        
            +
            Physical trauma (to self or others) after drinking alcohol
        
        
            +
            Pill-rolling tremor
        
        
            +
            Pinpoint spots on the soft palate
        
        
            +
            Pitting edema
        
        
            +
            Pleural effusion
        
        
            +
            Pleural friction rub
        
        
            +
            Polyps in nose
        
        
            +
            Poor concentration
        
        
            +
            Poor personal hygiene
        
        
            +
            Positive allergy challenge
        
        
            +
            Positive skin allergy (prick or patch) test
        
        
            +
            Poverty of speech
        
        
            +
            Premature graying
        
        
            +
            Preoccupation with food
        
        
            +
            Presence of neurofibromas
        
        
            +
            Pressing chest pain
        
        
            +
            Pretibial myxedema
        
        
            +
            Prior heart attack
        
        
            +
            Problems judging distances
        
        
            +
            Problems with everyday tasks such as washing, eating, dressing
        
        
            +
            Problems with movements coordination
        
        
            +
            Progressive hearing loss without fluctuation
        
        
            +
            Prolonged bleeding time
        
        
            +
            Prolonged erection
        
        
            +
            Prolonged exposure to sunlight and heat
        
        
            +
            Prolonged nighttime sleep
        
        
            +
            Prostate enlargement
        
        
            +
            Protruding eye or eyes
        
        
            +
            Psychological trauma
        
        
            +
            Psychomotor retardation
        
        
            +
            Pulsing headache
        
        
            +
            Pulsing sound in ears
        
        
            +
            Pupil dilation variation
        
        
            +
            Pupil or pupils unreactive to accomodation
        
        
            +
            Pus under the skin of the finger
        
        
            +
            Pyramidal signs
        
        
            +
            Quickening of thoughts and speech
        
        
            +
            Rash
        
        
            +
            Rash on face covering only the cheeks and the nose
        
        
            +
            Rash with small red or purple spots, not fading under pressure
        
        
            +
            Rash with small, red or purple spots, that can be felt, not fading under pressure
        
        
            +
            Rebound tenderness
        
        
            +
            Recent abdominal injury
        
        
            +
            Recent alcohol consumption
        
        
            +
            Recent head injury
        
        
            +
            Recent injury without bleeding
        
        
            +
            Recent physical injury
        
        
            +
            Recent surgery
        
        
            +
            Recent travel to Asia excluding Middle East, Russia, Mongolia and Kazakhstan
        
        
            +
            Recent travel to Australia and Oceania
        
        
            +
            Recent travel to Central Africa
        
        
            +
            Recent travel to Europe
        
        
            +
            Recent travel to Latin or South America
        
        
            +
            Recent travel to Northern Africa
        
        
            +
            Recent travel to Russia, Kazakhstan or Mongolia
        
        
            +
            Recent travel to Southern Africa
        
        
            +
            Recent travel to the Middle East
        
        
            +
            Recent travel to the United States or Canada
        
        
            +
            Recent vision problems
        
        
            +
            Receptive aphasia
        
        
            +
            Recurrent bleeding from the nose
        
        
            +
            Recurrent chest pain
        
        
            +
            Recurrent gastric ulcers
        
        
            +
            Recurrent stomach pain
        
        
            +
            Recurrent tonsils inflammation
        
        
            +
            Red and stinging eyes
        
        
            +
            Red and swollen oral mucosa
        
        
            +
            Red and warm skin change on eyelid
        
        
            +
            Red finger
        
        
            +
            Red gums
        
        
            +
            Red hand
        
        
            +
            Red lump with yellow tip on eyelid
        
        
            +
            Red patches on scalp
        
        
            +
            Red skin on arm or leg
        
        
            +
            Red skin on face
        
        
            +
            Red skin on palms
        
        
            +
            Red skin patches with silver-white, flaky scales
        
        
            +
            Red spots
        
        
            +
            Red stretch marks on abdomen, hips or thighs
        
        
            +
            Red throat
        
        
            +
            Red toe
        
        
            +
            Red, painful bumps on the skin overlying joints
        
        
            +
            Reddened foreskin or head of the penis
        
        
            +
            Reddened, swollen, and painful skin change
        
        
            +
            Redness and scaling affecting nearly entire skin
        
        
            +
            Redness around eyes
        
        
            +
            Redness behind the ear
        
        
            +
            Redness of skin
        
        
            +
            Redness on hips or sides of the thighs
        
        
            +
            Redness on the shoulders and the back of the neck
        
        
            +
            Reduced body temperature
        
        
            +
            Re-experiencing the traumatic event in nightmares
        
        
            +
            Re-experiencing the traumatic event in recurring distressing memories
        
        
            +
            Re-experiencing traumatic event
        
        
            +
            Religious delusion
        
        
            +
            Restricted passive range of joint motion
        
        
            +
            Restricted range of joint motion
        
        
            +
            Retinal artery occlusions
        
        
            +
            Retinal hemorrhage
        
        
            +
            Retinal microaneurysms
        
        
            +
            Retinal neovascularization elsewhere
        
        
            +
            Retinal swelling
        
        
            +
            Retrobulbar neuritis
        
        
            +
            Rheumatic fever
        
        
            +
            Ringing in ears
        
        
            +
            Ring-shaped skin changes
        
        
            +
            Risky drinking
        
        
            +
            Risky sexual behavior
        
        
            +
            Romberg's sign
        
        
            +
            Rose spots on lower chest and abdomen
        
        
            +
            Rossolimo's sign
        
        
            +
            Rovsing's sign
        
        
            +
            Ruddy, bulbous nose
        
        
            +
            Rumbling in ear
        
        
            +
            Runny nose
        
        
            +
            Runny or stuffed nose triggered by environment
        
        
            +
            Russell's sign
        
        
            +
            Sagging of the palate ipsilateral to suspected nerve damage
        
        
            +
            Salicylate overdose
        
        
            +
            Scarlet fever rash
        
        
            +
            Schaeffer's sign
        
        
            +
            Schizophasia
        
        
            +
            Scrotal transillumination
        
        
            +
            Seasonal pattern of depressive symptoms
        
        
            +
            Secret drinking
        
        
            +
            Sedative or hypnotic withdrawal
        
        
            +
            Seizures
        
        
            +
            Seizures with loss of consciousness
        
        
            +
            Seizures without loss of consciousness
        
        
            +
            Self-disgust after binge eating
        
        
            +
            Sense of self-worth dependent on weight
        
        
            +
            Series of headaches
        
        
            +
            Serious eye injury
        
        
            +
            Serious head injury in the last 5 years
        
        
            +
            Serious injury, respiratory difficulty, severe bleeding, new neurological problem
        
        
            +
            Serious skin wound
        
        
            +
            Severe and sudden coughing attacks
        
        
            +
            Severe back pain
        
        
            +
            Severe calf pain while walking
        
        
            +
            Severe chest pain
        
        
            +
            Severe facial pain
        
        
            +
            Severe headache
        
        
            +
            Severe joint pain after an injury
        
        
            +
            Severe pain after injury
        
        
            +
            Severe pain in lower limb
        
        
            +
            Severe pain in upper limb
        
        
            +
            Severe skin pain
        
        
            +
            Severe toothache
        
        
            +
            Sexual activity more often or with more partners than intended
        
        
            +
            Shafer's sign
        
        
            +
            Shallow breathing
        
        
            +
            Sharp and stabbing stomach pain
        
        
            +
            Shortness of breath
        
        
            +
            Shortness of breath appearing suddenly or developing over several minutes
        
        
            +
            Shortness of breath at rest
        
        
            +
            Shortness of breath getting worse over a few hours or days
        
        
            +
            Shortness of breath that comes and goes
        
        
            +
            Shortness of breath that worsens during exercise
        
        
            +
            Shortness of breath waking up at night
        
        
            +
            Shortness of breath when lying flat
        
        
            +
            Shoulder injury
        
        
            +
            Shoulder pain
        
        
            +
            Shoulder pain while moving
        
        
            +
            Shoulder swelling
        
        
            +
            Simple auditory paradelusions
        
        
            +
            Simple visual paradelusions
        
        
            +
            Single episode of bleeding from the nose
        
        
            +
            Sinus pain or pressure
        
        
            +
            Sinuses tender to palpation
        
        
            +
            Skin and blood vessel inflammation
        
        
            +
            Skin bump greater than 1 cm in diameter
        
        
            +
            Skin bump less than 1 cm in diameter
        
        
            +
            Skin change on eyelid
        
        
            +
            Skin change with rough, irregular surface
        
        
            +
            Skin changes
        
        
            +
            Skin changes aggravated by stress
        
        
            +
            Skin changes at the point of contact with buttons, fasteners, jewelry, or cosmetics
        
        
            +
            Skin changes forming a crust
        
        
            +
            Skin changes forming a line
        
        
            +
            Skin changes located on the face
        
        
            +
            Skin changes on feet
        
        
            +
            Skin changes on hands
        
        
            +
            Skin changes on oily areas of body
        
        
            +
            Skin changes on the entire body
        
        
            +
            Skin changes preceded by pain or itching
        
        
            +
            Skin changes that come and go
        
        
            +
            Skin changes worsening after alcohol consumption
        
        
            +
            Skin mass
        
        
            +
            Skin mass, bleeding
        
        
            +
            Skin mole or birthmark with diameter greater than 6 mm
        
        
            +
            Skin mole or birthmark with irregular border
        
        
            +
            Skin on arm or leg hot to the touch
        
        
            +
            Skin pain
        
        
            +
            Skin shedding
        
        
            +
            Skin thickening
        
        
            +
            Skull lesions
        
        
            +
            Sleep disorder
        
        
            +
            Slow and gradual stomach pain
        
        
            +
            Slow heart rate
        
        
            +
            Slowed movements
        
        
            +
            Slower walk
        
        
            +
            Small red spots on the skin, not fading under pressure
        
        
            +
            Small skin wound
        
        
            +
            Small white spots with red rims in the mouth, near the molar teeth
        
        
            +
            Smelly or colored waters
        
        
            +
            Smoking a lot
        
        
            +
            Smoking cigarettes
        
        
            +
            Smoking most cigarettes in the morning
        
        
            +
            Smooth face without wrinkles
        
        
            +
            Sneeze
        
        
            +
            Sneezing fits
        
        
            +
            Snoring
        
        
            +
            Social isolation
        
        
            +
            Social maladjustment
        
        
            +
            Soft lump in groin
        
        
            +
            Soft lump in inguinal region, enlarging during increase in abdominal pressure
        
        
            +
            Soft retinal exudates
        
        
            +
            Sore nail
        
        
            +
            Sore on the skin or mucous membrane
        
        
            +
            Sore throat
        
        
            +
            Spasms in upper extremities
        
        
            +
            Spastic dysarthria
        
        
            +
            Spending a lot of time on sex-related activity
        
        
            +
            Spider veins
        
        
            +
            Spider veins on the face
        
        
            +
            Spine asymmetry
        
        
            +
            Spine or joint stiffness subsiding after exercise
        
        
            +
            Splenic agenesis
        
        
            +
            Spoiled food or drink consumption
        
        
            +
            Squint with one or two eyes turning outward
        
        
            +
            Stabbing chest pain
        
        
            +
            Stabbing headache
        
        
            +
            Sterling's sign
        
        
            +
            Sternal rub
        
        
            +
            Stiff joints
        
        
            +
            Stiff neck
        
        
            +
            Stiffness of spine in the morning
        
        
            +
            Stimulants use (amphetamine, cocaine)
        
        
            +
            Stinging eyes and feeling of sand under eyelids
        
        
            +
            Stomach pain
        
        
            +
            Stomach pain after eating
        
        
            +
            Stomach pain decreases after bowel movement or passing gas
        
        
            +
            Stomach pain on deep inhalation and simultaneous pressing right upper part of abdomen
        
        
            +
            Stomach pain on empty stomach
        
        
            +
            Stomach pain worsening after caffeine consumption
        
        
            +
            Stomach pain worsens when coughing or moving
        
        
            +
            Stomach pain worsens with deep breath
        
        
            +
            Stomach pain, lasting 2 to 7 days
        
        
            +
            Stomach pain, lasting 8 to 14 days
        
        
            +
            Stomach pain, lasting less than two days
        
        
            +
            Stomach pain, lasting more than two weeks
        
        
            +
            Stransky's sign
        
        
            +
            Strawberry tongue
        
        
            +
            Stress-related gastrointestinal complaints
        
        
            +
            Stridor
        
        
            +
            Stroke
        
        
            +
            Strong physiological reactions while re-experiencing the traumatic event
        
        
            +
            Strumpell's sign
        
        
            +
            Stuffy nose
        
        
            +
            Sudden and sharp chest pain after an injury
        
        
            +
            Sudden back pain
        
        
            +
            Sudden breathing problems after injury
        
        
            +
            Sudden chest pain
        
        
            +
            Sudden crotch pain
        
        
            +
            Sudden face flushing
        
        
            +
            Sudden hearing loss
        
        
            +
            Sudden increase in blood pressure
        
        
            +
            Sudden joint pain
        
        
            +
            Sudden paleness of conjunctivae
        
        
            +
            Sudden paleness of mucous membranes
        
        
            +
            Sudden paleness of skin
        
        
            +
            Sudden stomach ache
        
        
            +
            Suicidal thoughts
        
        
            +
            Suicidal thoughts with plan and steps taken
        
        
            +
            Sunburn
        
        
            +
            Sunken eye
        
        
            +
            Sunken eyeballs
        
        
            +
            Suspicious mole or birthmark on the skin
        
        
            +
            Swallowed foreign body
        
        
            +
            Sweating on forehead on painful side
        
        
            +
            Swelling
        
        
            +
            Swelling around the eyes
        
        
            +
            Swelling behind ear
        
        
            +
            Swelling dependent on the body position
        
        
            +
            Swelling in the middle of the chest
        
        
            +
            Swelling inside the ear
        
        
            +
            Swelling of both hands
        
        
            +
            Swelling of nasal concha
        
        
            +
            Swelling of nasal mucosa
        
        
            +
            Swelling of temporal artery
        
        
            +
            Swelling of the upper limb
        
        
            +
            Swelling on both sides of throat
        
        
            +
            Swollen ankle
        
        
            +
            Swollen ankles
        
        
            +
            Swollen blue or bluish-gray nasal mucosa
        
        
            +
            Swollen cheek
        
        
            +
            Swollen feet
        
        
            +
            Swollen foot
        
        
            +
            Swollen gums
        
        
            +
            Swollen knee
        
        
            +
            Swollen leg
        
        
            +
            Swollen legs
        
        
            +
            Swollen lips
        
        
            +
            Swollen neck
        
        
            +
            Swollen nose
        
        
            +
            Swollen scrotum
        
        
            +
            Swollen toe
        
        
            +
            Swollen wrist
        
        
            +
            Symmetrical flaccid paresis in lower limbs (mainly feet)
        
        
            +
            Symmetrical flaccid paresis in upper limbs (mainly hands)
        
        
            +
            Symmetrical paresthesias
        
        
            +
            Symptoms appear after taking painkillers, change in diet
        
        
            +
            Symptoms do not occur during sleep
        
        
            +
            Symptoms exacerbated by cold temperature
        
        
            +
            Symptoms increase in evening
        
        
            +
            Symptoms occur several hours after drinking excessive alcohol
        
        
            +
            Tactile hallucinations
        
        
            +
            Taking sleeping pills or sedatives
        
        
            +
            Taste disorder in anterior two-thirds of tongue
        
        
            +
            Temporary blindness
        
        
            +
            Tender leg
        
        
            +
            Tender lower leg
        
        
            +
            Terry's nails
        
        
            +
            Thick eye discharge
        
        
            +
            Thickened and cracked skin on hands
        
        
            +
            Thickened nails
        
        
            +
            Thickened nails with pits or grooves
        
        
            +
            Thigh injury
        
        
            +
            Thigh pain
        
        
            +
            Throckmorton's sign
        
        
            +
            Thumb pain
        
        
            +
            Thumb pain while moving
        
        
            +
            Time spent at more than 2500 m above sea level
        
        
            +
            Tingling around the mouth
        
        
            +
            Tingling fingers
        
        
            +
            Tingling in the ring and little finger
        
        
            +
            Tingling in the thumb, index finger, and middle finger
        
        
            +
            Tiny eggs at the base of the hair shaft
        
        
            +
            Toe injury
        
        
            +
            Toe pain
        
        
            +
            Tongue enlargement
        
        
            +
            Tooth loss
        
        
            +
            Toothache
        
        
            +
            Toothache after contact with cold, sweet, or hot food
        
        
            +
            Toothache while chewing
        
        
            +
            Toothache worsened by heat
        
        
            +
            Transverse furrow on the nose
        
        
            +
            Trembling at rest
        
        
            +
            Trembling during movement
        
        
            +
            Trembling eyelids
        
        
            +
            Trembling of both hands
        
        
            +
            Trembling of one hand
        
        
            +
            Trembling of tongue
        
        
            +
            Trembling subsiding after alcohol
        
        
            +
            Trembling worsening after caffeine
        
        
            +
            Trembling worsening under stress
        
        
            +
            Trigeminal neuralgia
        
        
            +
            Tromner's sign
        
        
            +
            Trophic changes
        
        
            +
            Trophic ulcer
        
        
            +
            Trophic ulcer on the plantar side of the foot
        
        
            +
            Trouble beginning urination
        
        
            +
            Trouble planning the future
        
        
            +
            Trousseau sign of latent tetany
        
        
            +
            Trunk injury
        
        
            +
            Tunnel Vision
        
        
            +
            Twisted neck
        
        
            +
            Tympanic membrane light reflex distortion
        
        
            +
            Ulcerations or abscesses near the anus
        
        
            +
            Unable to urinate
        
        
            +
            Unbearable dizziness
        
        
            +
            Unbearable stomach pain
        
        
            +
            Unspecified conditions marker
        
        
            +
            Unstable knee
        
        
            +
            Unusually red tongue
        
        
            +
            Unusually red tonsils
        
        
            +
            Upper central stomach pain
        
        
            +
            Upper left side stomach pain
        
        
            +
            Upper limb injury
        
        
            +
            Upper right side stomach pain
        
        
            +
            Urgent need to urinate
        
        
            +
            Urinary incontinence
        
        
            +
            Urinating at night
        
        
            +
            Urination in small amounts
        
        
            +
            Urination while coughing or laughing or during physical exertion
        
        
            +
            Using methods to lose weight: vomiting, starvation diets
        
        
            +
            Uvula deviation towards unaffected side
        
        
            +
            Verbal learning disability
        
        
            +
            Vesicles filled with pus, up to 1 cm in diameter
        
        
            +
            Violating social norms after drinking alcohol
        
        
            +
            Visible and palpable enlarged vein of limb, chest or abdomen
        
        
            +
            Visible head lice on scalp
        
        
            +
            Visual agnosia
        
        
            +
            Visual field loss on the same side of both eyes
        
        
            +
            Visual hallucinations
        
        
            +
            Vitreous hemorrhage
        
        
            +
            Vomiting
        
        
            +
            Vomiting after every meal
        
        
            +
            Vomiting blood
        
        
            +
            Vomiting more often in the morning
        
        
            +
            Vomiting that ejects with great force
        
        
            +
            V-shaped redness on the chest
        
        
            +
            Waist-hip ratio (WHR) over 1.0 in men
        
        
            +
            Waking up early
        
        
            +
            Watery eyes
        
        
            +
            Weak eye clenching
        
        
            +
            Weak or absent pharyngeal reflex
        
        
            +
            Weak peripheral pulses
        
        
            +
            Weak pulse in legs
        
        
            +
            Weakness of the half of the body
        
        
            +
            Weakness worsening after exercise
        
        
            +
            Weight gain
        
        
            +
            Weight loss without loss of appetite
        
        
            +
            Weight reduction
        
        
            +
            Weiss ring
        
        
            +
            Westphal's sign
        
        
            +
            Whiplash
        
        
            +
            Whistling sound made while breathing
        
        
            +
            White nail discoloration
        
        
            +
            White or yellow formations in tonsils
        
        
            +
            White patch on cornea
        
        
            +
            White patches on the oral mucous membrane
        
        
            +
            White spots on tonsils
        
        
            +
            Widened neck veins
        
        
            +
            Widespread chest pain
        
        
            +
            Widespread stomach pain
        
        
            +
            Wild mushroom consumption
        
        
            +
            Winterbottom's sign
        
        
            +
            Woman
        
        
            +
            Working too much
        
        
            +
            Worms in stool
        
        
            +
            Worse functioning at home, work, or school
        
        
            +
            Worst headache in life
        
        
            +
            Wound
        
        
            +
            Wound infection
        
        
            +
            Wrist drop
        
        
            +
            Wrist injury
        
        
            +
            Wrist pain
        
        
            +
            Wrist pain while moving
        
        
            +
            Yellow eyes
        
        
            +
            Yellow nails
        
        
            +
            Yellow or green discharge from ear
        
        
            +
            Yellow skin lesions on eyelids
        
        
            +
            Yellowed skin
        
        
            +
            Yellowed skin after a physical exertion
        
        
            +
            Yellowed skin after a time of increased stress
        
        
            +
            Yellowed skin while suffering from an infectious disease
        
        
            +
            Yellowish deposits under skin
        

    
        pregnantRiskId = '';
        postPartumRiskId = '';
        $(document).ready(function () {
            $(&quot;.evidence-item&quot;).on('click', toggleReported);
            //$(&quot;.evidence-item > button&quot;).on('click', toggleReported);
            
            
        });

    

                            

                        
                    
                    
                        
                            Next
                        
                    

                


            

        
        


    
        
            
                
                    
                        Reported
                    
                
            

            

                Nothing has been reported yet

                
                    
                        
                            Present: (0)
                        
                    
                    
                        
                        
                    

                    
                        
                            Absent: (0)
                        
                    

                    
                        
                        
                    
                    
                    
                        
                            Unknown: (0)
                        
                    

                    
                        
                        
                    

                


            
            

        

    
    
    
        
    

    

        .mb-0 > a {
            display: block;
            position: relative;
            color: black;
        }

            .mb-0 > a:after {
                content: &quot;\f0da&quot;;
                font-family: 'FontAwesome';
                position: absolute;
                left: 0;
            }

            .mb-0 > a[aria-expanded=&quot;true&quot;]:after {
                content: &quot;\f0d7&quot;;
            }
    




    $('.tooltipDisplayable').tooltipster({
        theme: 'tooltipster-noir'
    });

    





    var arr = [];

    var loadingSymptoms = 'Loading Symptoms';
    var addSymptom = 'Add the symptoms here';
    var selectPrimary = 'Please select a primary concept';
    var refreshingSymptoms = 'Refreshing symptoms';
    var typeSymptom = 'Type in a Symptom';
    var cleared = 'False';
    var selectPresentSymptomWarning = 'Select a present symptom to continue';
    var oneSymptomWarning = 'Please note that only one symptom was selected, &amp;#xD;&amp;#xA;by doing so you run the risk of an extended interview process. &amp;#xD;&amp;#xA;Do you wish to continue?';
    var symptoms = 'Symptoms';
    var yes = 'Yes';
    var no = 'No';


    var oneSymptomWarning = 'Please note that only one symptom was selected, &amp;#xD;&amp;#xA;by doing so you run the risk of an extended interview process. &amp;#xD;&amp;#xA;Do you wish to continue?';
    $(document).ready(function () {

        jQuery.expr[&quot;:&quot;].Contains = jQuery.expr.createPseudo(function(arg) {
            return function( elem ) {
                return jQuery(elem).text().toUpperCase().indexOf(arg.toUpperCase()) >= 0;
            };
        });

        GetTerms(cleared);
        
        $('#btnClearSympt').tooltipster({
            theme: 'tooltipster-noir'
        });

        ClearHeader();

        var timer = null;
        $('#Symptoms').keydown(function(){
            clearTimeout(timer);
            timer = setTimeout(filterSymptoms, 500);
        });

        function filterSymptoms() {
            var filter = $('#Symptoms').val();
            if ($.trim(filter) === '') {
                $('#divAllSymptomsContainer ul').find('li[data-common=&quot;1&quot;]').show();
                $('#divAllSymptomsContainer ul').find('li[data-common=&quot;0&quot;]').hide();
                $('#selectableSimptomLabel').css('display', 'block');

            } else {
                $('#selectableSimptomLabel').css('display', 'none');
                $('#risk-factors').collapse('show');

                $('#divAllSymptomsContainer ul').find('li[data-common=&quot;0&quot;]').show();
                $('#divAllSymptomsContainer ul').find('li[data-common=&quot;1&quot;]').show();

                $(&quot;#divAllSymptomsContainer ul > li&quot;).hide();
                $(&quot;#divAllSymptomsContainer ul > li:Contains('&quot; + filter + &quot;')&quot;).show();
            }
        }

        
    });

    




    $(document).ready(function() {
        ShowAnimation('bodymovin', '/animation/home_main.json');

        //$('#btnYesEmer').bind('click', LoadEmergencyTriage);
        $('#btnYesEmer').on('click', LoadEmergencyTriage);
        //$('#btnNoEmer').bind('click', LoadIsDisclaimer);
        $('#btnNoEmer').on('click', LoadIsDisclaimer);
    });


















    var actionGetIsEmergency = '/en-US/Home/GetIsEmergency';
    var actionGetDisclaimerApproval = '/en-US/Home/GetDisclaimerApproval';
    var actionGetMainTriage = '/en-US/Home/GetMainTriage';
    var actionGetLocation= '/en-US/Home/GetLocation';
    var actionGetSymptoms = '/en-US/Home/GetSymptoms';

    var actionUpdateLogLocation = '/en-US/Home/UpdateLogLocation';

    var actionGetSymptomsText = '/en-US/Home/GetSymptomsText';
    var actionGetCoordination = '/en-US/Home/GetCoordination';
    var actionGetContacts = '/en-US/Home/GetContacts';
    var actionSaveNotes = '/en-US/Home/SaveNote';
    var actionGetEmergencyTriage = '/en-US/Home/GetEmergencyTriage';
    var actionGetEmergencyCoordination = '/en-US/Home/GetEmergencyCoordination';
    var actionGetFacilityCoordination = '/en-US/Home/GetFacilityCoordination';
    var actionBackToPredefinedConditions = '/en-US/Home/BackToPredefinedConditions';
    var actionGetFinalPreview = '/en-US/Home/GetFinalPreview';
    var actionSearchClaim = '/en-US/Home/SearchClaim';

    var actionGetAddContact = '/en-US/Home/GetAddContact';
    var actionAddContact = '/en-US/Home/AddUpdateContact';
    var actionRemoveContact = '/en-US/Home/RemoveContact';

    var actionGetTerms = '/en-US/Home/GetTerms';
    var actionStoreSelectedTerms = '/en-US/Home/StoreSelectedTerms';

    var actionGetSymptomsTerms = '/en-US/Home/GetSymptomsTerms';
    var actionDeleteSelectedTerm = '/en-US/Home/DeleteSelectedTerm';
    var actionIndex = '/';
    var actionAllowedToAddPatient = '/en-US/Home/AllowedToAddPatient';
    var actionHouseCallCoordination= '/en-US/Home/HouseCallCoordination';
    var actionFacilityCoordination= '/en-US/Home/FacilityCoordination';
    var actionGetUpdateLocation = '/en-US/Home/GetUpdateLocation';
    var actionUpdateLocation = '/en-US/Home/UpdateLocation';
    var actionSubmit = '/en-US/Home/Submit';
    var actionSaveTelemedicineCoordination = '/en-US/Home/SaveTelemedicineCoordination';
    var actionAddPharmaContact = '/en-US/Home/AddPharmaContact';
    var actionAddVendorContact = '/en-US/Home/AddVendorContact';
    var actionAddHouseCallContact = '/en-US/Home/AddHouseCallContact';
    var actionAddFacilityContact = '/en-US/Home/AddFacilityContact';
    var actionGetSubOrgs = '/en-US/Home/GetSubOrgs';
    var actionGetCurrentTime = '/en-US/Home/GetZonedTime';
    var actionGetStates = '/en-US/Home/GetStates';
    var actionGetCities = '/en-US/Home/GetCities';
    var actionGetJsonZonedTime = '/en-US/Home/GetJsonZonedTime';
    var actionUpdateAddressLocation = '/en-US/Home/UpdateAddressLocation';
    var actionGetSynonyms = '/en-US/Home/GetSynonyms';

    var actionExistsEmailFax = '/en-US/Home/ExistsEmailFax';
    var actionBackToEmergencyCoordination = '/en-US/Home/BackToEmergencyCoordination';
    var actionGetJsonLtLnTime = '/en-US/Home/GetJsonLtLnTime';
    var actionReOrderLocations = '/en-US/Home/ReOrderLocations';
    var actionRefreshFacility = '/en-US/Home/RefreshFacility';
    var actionReportBug = '/en-US/Home/ReportBug';
    var actionSearchLinkClaim = '/en-US/Home/SearchLinkClaim';
    var actionSearchLinkClaimDetail = '/en-US/Home/SearchLinkClaimDetail';
    var actionShowLinkClaim= '/en-US/Home/ShowLinkClaim';
    var actionSendBug = '/en-US/Home/SendBug';
    var actionLinkClaimFinal= '/en-US/Home/LinkClaimFinal';
    var actionSearchClaimMemberData = '/en-US/Home/SearchClaimMemberData';
    var actionShowHeader = '/en-US/Home/ShowHeader';
    var actionGetHouseVisitAddress = '/en-US/Home/GetHouseVisitAddress';
    var actionGetPatientAddressHouseCall = '/en-US/Home/GetPatientAddressHouseCall';
    var actionValidatePatientAddressHouseCall = '/en-US/Home/ValidatePatientAddressHouseCall';
    var actionUpdatePatientAddressHouseCall = '/en-US/Home/UpdatePatientAddressHouseCall';
    var actionLoadPharma = '/en-US/Home/LoadPharma';
    var actionGetFacilities = '/en-US/Home/GetFacilities';
    var actionRefreshCoordination = '/en-US/Home/RefreshCoordination';
    var actionAuthorizedCountriesPerRole = '/en-US/Home/AuthorizedCountriesPerRole';
    var actionGetMemberDetails = '/en-US/Home/GetMemberDetails';
    var actionSaveMemberDetails = '/en-US/Home/SaveMemberDetails';
    var actionCleanMemberData = '/en-US/Home/CleanMemberData';
    var actionSignOut = '/en-US/Home/SignOut';
    var actionGetCareDeclined = '/en-US/Home/GetCareDeclined';
    var actionCareDecline = '/en-US/Home/CareDecline';
    var actionPharmacyLinkClicked = '/en-US/Home/PharmacyLinkClicked';
    var actionClearSymptomsTerm = '/en-US/Home/ClearSymptomsTerm';
    var actionOpenCitySearch = '/en-US/Home/OpenCitySearch';
    var actionCitySearch = '/en-US/Home/CitySearch';
    var actionCheckACMOrganization = '/en-US/Home/CheckACMOrganization';
    var actionCheckACMMemberDetails = '/en-US/Home/CheckACMMemberDetails';
    var actionGetTimeUntilExpiration = '/en-US/Home/GetTimeUntilExpiration';
    var actionExtendSession = '/en-US/Home/ExtendSession';
    var actionCleanFinalCache = '/en-US/Home/CleanFinalCache';

    var actionGetCacheTerms = '/en-US/Home/GetCacheTerms';


    var actionRemoveDeclinedFlag = '/en-US/Home/RemoveDeclinedFlag';
    var actionCheckMemberDetails = '/en-US/Home/CheckMemberDetails';
    var actionReportTwo = '/en-US/Report/VCardPageTwo';
    var actionReportOne = '/en-US/Report/VCardPageOne';
    var actionClientSummary = '/en-US/Home/ClientSummary';
    var actionPartialClientSummary = '/en-US/Home/PartialClientSummary';
    var actionUpdateClientSummaryContact = '/en-US/Home/UpdateClientSummaryContact';
    var actionPossibleCauses = '/en-US/Home/GetPossibleCauses';
    var actionPossibleCausesDetails = '/en-US/Home/GetPossibleCausesDetails';
    var actionGetHeaderData = '/en-US/Home/GetHeaderData';
    var actionPreLoadTerms = '/en-US/Home/PreLoadTerms';
    var actionFullReport = '/en-US/Report/FullReport';
    var actionCheckHVAvailability = '/en-US/Home/CheckHVAvailability';
    var actionGetResend = '/en-US/Home/GetResend';
    var actionResend= '/en-US/Home/Resend';
    var actionGetTelemedicineScript = '/en-US/Home/GetTelemedicineScript';
    var actionCustomizableTelemedicineValidation = '/en-US/Home/CustomizableTelemedicineValidation'
    var actionGetAdministrationUrl = '/en-US/Home/GetAdministrationUrl';
    var actionGetAdministrationLevel = '/en-US/Home/GetAdministrationLevel';
    var actionGetAgeRiskFactors = '/en-US/Home/GetAgeRiskFactors';
    var actionGetSelectableSymptoms = '/en-US/Home/GetSelectableSymptoms';
    var actionGetRedFlags = '/en-US/Home/GetRedFlagsView';
    var actionGetReportedLine = '/en-US/Home/GetReportedLine';
    var actionGetRelated = '/en-US/Home/GetRelated';
    var actionSaveRelatedGetInterviewMain = '/en-US/Home/SaveRelatedGetInterviewMain';
    var actionGetInterviewProcessQuestions = '/en-US/Home/GetInterviewProcessQuestions';
    var actionGetQuestionByOrder = '/en-US/Home/GetQuestionByOrder';
    var actionShowDisclaimer = '/en-US/Home/ShowDisclaimer';
    var actionGetFlagFacility = '/en-US/Home/GetFlagFacility';

    var actionConfirmFlagFacility = '/en-US/Home/ConfirmFlagFacility';
    var actionGetTelemedicineAvailable = '/en-US/Home/GetTelemedicineAvailable';
    var actionGetPreCoordination = '/en-US/Home/GetPreCoordination';
    var actionGetTelemedicineDeclined = '/en-US/Home/GetTelemedicineDeclined';
    var actionTelemedicineDecline = '/en-US/Home/TelemedicineDecline';
    var actionGetPossibleConditions = '/en-US/Home/GetPossibleConditions';
    var actionGetPreSubmit = '/en-US/Home/GetPreSubmit';
    var actionSetTriageDateAndTime = '/en-US/Home/SetTriageDateAndTime';
    var actionGetCareDeclinedSubOptions = '/en-US/Home/GetCareDeclinedSubOptions';
    var actionUpdateScreenResolution = '/en-US/Home/UpdateScreenResolution';
    var actionEndSession= '/en-US/Home/EndSession';


    var searchBox;
    var tryAgain = 'Try Again!';
    var systemError = 'Click OK and try again, if the issue persists please report the issue.';
    var opNotAuth = 'Operation not authorized';
    var cancelSession = 'Cancel Session';
    var cancelSessionQuestion = 'Are you sure you wish to cancel this triage? All data will be lost.';
    var completeAllFields = 'Please complete all the fields';
    var linkClaim = 'Link Claim';
    var claimLinkSuccesful = 'The claim was linked succesfully';

    var sessionAboutExpire = 'Session is about to expire';
    var wishSessionExtension = 'Do you want to extend the session?';
    var yes = 'Yes';
    var no = 'No';
    var sessionExtended = 'Session extended';
    var version = '4.11.0-beta.62';
    var status = '0';


    var memberDetails = 'Member Details';
    var subOrgMandatory = 'Please complete all member detail fields in order to continue';
    var isAdmin = 'False'; //hay que mostrar u ocultar btnSubmit segun corresponda
    var isDebug = 'False'; //hay que mostrar u ocultar btnSubmit segun corresponda



    var menuPatient = {
        Member: '',
        Age: '',
        Dob: '',
        Gender: '',
        Location: {
            FullAddress: '',
            City: '',
            State: '',
            Time: ''
        }
    };
    var pregnantRiskId = '' ;
    var postPartumRiskId = '';

    $(document).ready(function () {

        GetCacheTerms();
        $('#hdnStarted').val(0);
        $('#lblVersion').html('Version ' + version);

        var admin = (/true/i).test(isAdmin);
        if (admin) {
            $('#btnSubmit').css('display', 'inline-block');
        } else {
            $('#btnSubmit').css('display', 'none');
        }


        var $debug = (/true/i).test(isDebug);

        if ($debug) {
            $('#divDebugAddressAndDate').css('display', 'block');
            var today = new Date(new Date().getFullYear(), new Date().getMonth(), new Date().getDate());
            debugger;
            $('#triageDateAndTime').datetimepicker({
                format:'mm/dd/yyyy HH:MM'
            });
        } else {
            $('#divDebugAddressAndDate').css('display', 'none');
        }


        window.onbeforeunload = function() {
            //alert('closing');
            //return false;
            debugger;
            EndSession();
            //if we return nothing here (just calling return;) then there will be no pop-up question at all
            //return;
        };

        setInterval(UpdateHeaderTimeZone, 60000);

        setInterval(checkTimeoutSession, 60000);


        var isMobile = false; //initiate as false
        // device detection
        if (/(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|ipad|iris|kindle|Android|Silk|lge |maemo|midp|mmp|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows (ce|phone)|xda|xiino/i.test(navigator.userAgent)
            || /1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s\-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|\-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw\-(n|u)|c55\/|capi|ccwa|cdm\-|cell|chtm|cldc|cmd\-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc\-s|devi|dica|dmob|do(c|p)o|ds(12|\-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(\-|_)|g1 u|g560|gene|gf\-5|g\-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd\-(m|p|t)|hei\-|hi(pt|ta)|hp( i|ip)|hs\-c|ht(c(\-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i\-(20|go|ma)|i230|iac( |\-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc\-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|\-[a-w])|libw|lynx|m1\-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m\-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(\-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)\-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|\-([1-8]|c))|phil|pire|pl(ay|uc)|pn\-2|po(ck|rt|se)|prox|psio|pt\-g|qa\-a|qc(07|12|21|32|60|\-[2-7]|i\-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h\-|oo|p\-)|sdk\/|se(c(\-|0|1)|47|mc|nd|ri)|sgh\-|shar|sie(\-|m)|sk\-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h\-|v\-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl\-|tdg\-|tel(i|m)|tim\-|t\-mo|to(pl|sh)|ts(70|m\-|m3|m5)|tx\-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|\-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(\-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas\-|your|zeto|zte\-/i.test(navigator.userAgent.substr(0, 4))) {
            isMobile = true;
        }
        if (isMobile) {
            $('#divMainDesktop').css('display', 'none');
            $('#divMainMobile').css('display', 'block');

        } else {


            LoadHeader('');

            $(&quot;#ClaimNumber&quot;).keydown(function (e) {
                // Allow: backspace, delete, tab, escape, enter and .
                if ($.inArray(e.keyCode, [46, 8, 9, 27, 13]) !== -1 ||
                    // Allow: home, end, left, right, down, up
                    (e.keyCode >= 35 &amp;&amp; e.keyCode &lt;= 40)) {
                    // let it happen, don't do anything
                    return;
                }
                // Ensure that it is a number and stop the keypress
                if ((e.shiftKey || (e.keyCode &lt; 48 || e.keyCode > 57)) &amp;&amp; (e.keyCode &lt; 96 || e.keyCode > 105)) {
                    e.preventDefault();
                }
            });


            $('#ClaimNumber').keypress(function (e) {
                if (e.which == 13) {
                    searchClaim();
                }
            });

            if (status == 20) {
                $('#divTopHeader').html('');
            }
        }


        if (window.appInsights.queue != undefined) {
            window.appInsights.queue.push(function () {

                var userName = 'gem3@globalexcel.com';

                appInsights.context.user.authenticatedId = userName;
                appInsights.context.user.accountId = userName;
                appInsights.context.user.id = userName;
            });

        }

        var $height = $(window).height();
        var $width = $(window).width();


        UpdateScreenResolution('30146', $height, $width);

    });
    function logout() {
        window.location = actionSignOut;
    }



        
            
                
                    Triage Date and time:
                
                
                    
                
            
            
                
                
                    
                
                
            

        

        
        
    

    








    var logOut = 'Log out';
    var logoutQuestion = 'Are you sure you want to log out?';
    var yes = 'Yes';
    var no = 'No';
    $('#logout').tooltipster({
        theme: 'tooltipster-noir'
    });

    $('#reportError').tooltipster({
        theme: 'tooltipster-noir'
    });
    GetAdministrationLevel();


    function ValidateLogout() {
        $.confirm({
            title: logOut,
            content: logoutQuestion,
            buttons: {
                Yes: {
                    text: yes,
                    btnClass: 'btn-success',
                    action: function () {
                        window.location = actionSignOut;

                    }
                },
                No: {
                    text: no,
                    btnClass: 'btn-danger',
                    action: function () {
                        return;
                    }
                }
            }
        });
        return false;
    }

/html[1]/body[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
</WebElementEntity>
