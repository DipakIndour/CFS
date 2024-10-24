import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.amazon.in/?&tag=googhydrabk1-21&ref=pd_sl_5szpgfto9i_e&adgrpid=155259813593&hvpone=&hvptwo=&hvadid=713930225169&hvpos=&hvnetw=g&hvrand=4181849861770463019&hvqmt=e&hvdev=c&hvdvcmdl=&hvlocint=&hvlocphy=9061994&hvtargid=kwd-64107830&hydadcr=14452_2402225&gad_source=1')

WebUI.setText(findTestObject('Object Repository/amazon/Page_Online Shopping site in India Shop Onl_10c5f3/input_Search Amazon.in_field-keywords'), 
    'https://www.amazon.in/Acer-inches-Ultra-Google-AR50UDIGU2875AT/dp/B0D9BY8GH3?pd_rd_w=lCBbb&content-id=amzn1.sym.4208c4b0-bca1-4d25-819d-4a63e3b5b520&pf_rd_p=4208c4b0-bca1-4d25-819d-4a63e3b5b520&pf_rd_r=KQXSR3SK3Y2J8M279YNR&pd_rd_wg=zdNvB&pd_rd_r=ad2e22aa-d1cc-42b9-b5d0-d8f69622cffa&pd_rd_i=B0D9BY8GH3&ref_=pd_hp_d_btf_unk_B0D9BY8GH3')

WebUI.click(findTestObject('Object Repository/amazon/Page_Online Shopping site in India Shop Onl_10c5f3/input_Search Amazon.in_nav-search-submit-button'))

WebUI.click(findTestObject('Object Repository/amazon/Page_Amazon.in  httpswww.amazon.inAcer-inch_e69681/input_Search Amazon.in_nav-search-submit-button'))

